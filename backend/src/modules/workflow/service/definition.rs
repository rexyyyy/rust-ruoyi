//! 流程定义。对标 Java `FlwDefinitionServiceImpl` + warm-flow 的 `DefService`。
//!
//! 设计器保存的是「定义 + 节点 + 连线」的整份 JSON，这里只做
//! 「参数整形 → 调引擎 → 回查最新版本」的薄封装。

use std::collections::HashMap;

use sea_orm::ConnectionTrait;
use warm_flow_core::dto::DefJson;
use warm_flow_core::store::DefinitionQuery;
use warm_flow_web::UiExtension;

use crate::error::{Error, Result};
use crate::modules::workflow::domain::bo::DefinitionPageBo;
use crate::modules::workflow::domain::vo::DefinitionVO;
use crate::modules::workflow::domain::Page;
use crate::modules::workflow::service::{self, flow_err};
use crate::modules::workflow::ui;

/// 流程定义分页
pub async fn page(bo: &DefinitionPageBo) -> Result<Page<DefinitionVO>> {
    let engine = service::engine()?;
    let page_no = bo.page_no.unwrap_or(1).max(1);
    let page_size = bo.page_size.unwrap_or(10).clamp(1, 200);
    let query = DefinitionQuery {
        flow_code: service::blank_to_none(bo.flow_code.as_deref()),
        flow_name: service::blank_to_none(bo.flow_name.as_deref()),
        category: service::blank_to_none(bo.category.as_deref()),
        is_publish: bo.is_publish,
        page_num: page_no,
        page_size,
    };
    let data = engine.list_definitions(&query).await.map_err(flow_err)?;
    let names = category_names().await;
    let rows: Vec<DefinitionVO> = data
        .rows
        .into_iter()
        .map(|def| {
            let mut vo = DefinitionVO::from(def);
            vo.category_name = vo
                .category
                .as_deref()
                .and_then(|c| names.get(c).cloned());
            vo
        })
        .collect();
    Ok(Page::new(rows, data.total, page_no, page_size))
}

/// 详情（含分类名）
pub async fn detail(id: i64) -> Result<DefinitionVO> {
    let engine = service::engine()?;
    let def = engine.get_all_data_definition(id).await.map_err(flow_err)?;
    let mut vo = DefinitionVO::from(def);
    vo.category_name = vo
        .category
        .as_deref()
        .and_then(|c| category_name_from_cache(c));
    Ok(vo)
}

/// 已发布的流程定义下拉（发起流程时选择）
pub async fn published_list() -> Result<Vec<DefinitionVO>> {
    let engine = service::engine()?;
    let query = DefinitionQuery {
        is_publish: Some(1),
        page_num: 1,
        page_size: 200,
        ..Default::default()
    };
    let data = engine.list_definitions(&query).await.map_err(flow_err)?;
    Ok(data.rows.into_iter().map(DefinitionVO::from).collect())
}

/// 设计器数据：`id` 为空时返回一个带默认值与分类树的新建壳子
pub async fn design(id: Option<i64>) -> Result<DefJson> {
    let engine = service::engine()?;
    let mut def_json = match id {
        Some(id) => engine.query_design(id).await.map_err(flow_err)?,
        None => DefJson::default(),
    };
    // 分类树：设计器「分类」级联选择
    let categories = ui::ui().categories();
    if !categories.is_empty() {
        def_json.category_list = Some(categories);
    }
    let paths = ui::ui().form_paths();
    if !paths.is_empty() {
        def_json.form_path_list = Some(paths);
    }
    def_json.chart_status_color = Some(engine.chart_rgb(&def_json.model()));
    def_json.top_text_show = Some(engine.config().top_text_show);
    Ok(def_json)
}

/// 设计器 JSON 的 Value 形态
///
/// `DefJson.id` 是 `Option<i64>`，直接序列化成数字会被 JS 截断精度
/// （19 位雪花 id > Number.MAX_SAFE_INTEGER），存盘时就会写回一个错误的定义。
/// 这里统一改成字符串，入参再宽容地接回来。
pub async fn design_value(id: Option<i64>) -> Result<serde_json::Value> {
    let def_json = design(id).await?;
    let mut value = serde_json::to_value(def_json).map_err(|e| Error::from(e.to_string()))?;
    stringify_id(&mut value);
    Ok(value)
}

/// 保存设计器 JSON（Value 形态，id 允许是字符串）
pub async fn save_value(body: &serde_json::Value, only_node_skip: bool) -> Result<DefinitionVO> {
    let mut value = body.clone();
    parse_id(&mut value)?;
    let def_json: DefJson =
        serde_json::from_value(value).map_err(|e| Error::from(format!("流程定义 JSON 解析失败: {e}")))?;
    save(&def_json, only_node_skip).await
}

/// 出参：数字 id → 字符串
fn stringify_id(value: &mut serde_json::Value) {
    if let Some(obj) = value.as_object_mut() {
        if let Some(id) = obj.get("id").and_then(|v| v.as_i64()) {
            obj.insert("id".to_string(), serde_json::Value::String(id.to_string()));
        }
    }
}

/// 入参：字符串 id → 数字（空串视为新建）
fn parse_id(value: &mut serde_json::Value) -> Result<()> {
    let Some(obj) = value.as_object_mut() else {
        return Ok(());
    };
    let Some(raw) = obj.get("id").and_then(|v| v.as_str()).map(str::trim).map(str::to_string) else {
        return Ok(());
    };
    if raw.is_empty() {
        obj.remove("id");
        return Ok(());
    }
    let id = raw
        .parse::<i64>()
        .map_err(|_| Error::from(format!("流程定义 id 不合法: {raw}")))?;
    obj.insert("id".to_string(), serde_json::json!(id));
    Ok(())
}

/// 保存设计器 JSON；`only_node_skip = true` 时只覆盖节点与连线
pub async fn save(def_json: &DefJson, only_node_skip: bool) -> Result<DefinitionVO> {
    let engine = service::engine()?;
    let flow_code = def_json
        .flow_code
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| Error::from("流程编码（flowCode）不能为空"))?
        .to_string();
    // 引擎的更新分支是「整行覆写」，而 `DefJson` 里根本没有 createTime 字段，
    // `to_definition()` 出来就是 None，于是每次保存都会把库里的创建时间刷成 NULL
    // （流程定义列表「创建时间」空白）。这里先记下原值，保存后补回去。
    let keep_create_time = match (def_json.id, only_node_skip) {
        (Some(id), false) => detail(id).await.ok().and_then(|vo| vo.create_time),
        _ => None,
    };
    let saved = engine
        .save_def(def_json, only_node_skip)
        .await
        .map_err(flow_err)?;
    // 引擎保存后返回的是入参回显（新建时 id 为空），这里回查最新版本，保证前端拿到真实 id
    let latest = match saved.id {
        Some(id) => id,
        None => latest_id(&flow_code).await?,
    };
    if let Some(ts) = keep_create_time {
        restore_create_time(latest, ts).await?;
    }
    detail(latest).await
}

/// 把被引擎覆写成 NULL 的创建时间补回去
async fn restore_create_time(id: i64, ts: chrono::NaiveDateTime) -> Result<()> {
    let db = crate::db::db().ok_or_else(|| Error::from("数据库未初始化"))?;
    db.execute(crate::db::raw_stmt(
        "UPDATE flow_definition SET create_time = $1 WHERE id = $2 AND create_time IS NULL",
        vec![sea_orm::Value::from(ts), sea_orm::Value::from(id)],
    ))
    .await
    .map_err(|e| Error::from(format!("补回流程定义创建时间失败: {e}")))?;
    Ok(())
}

/// 按 flowCode 回查最新一条定义的 id
///
/// 引擎的 `save_def`/`import_json` 新建分支返回的是入参回显，`id` 为空，
/// 所以凡是「新建后需要主键」的地方都走这里兜底。
async fn latest_id(flow_code: &str) -> Result<i64> {
    let engine = service::engine()?;
    let data = engine
        .list_definitions(&DefinitionQuery {
            flow_code: Some(flow_code.to_string()),
            page_num: 1,
            page_size: 200,
            ..Default::default()
        })
        .await
        .map_err(flow_err)?;
    data.rows
        .into_iter()
        .filter(|d| d.flow_code == flow_code)
        // 雪花 id 单调递增，create_time 相同也能排稳
        .max_by_key(|d| (d.create_time, d.id.unwrap_or_default()))
        .and_then(|d| d.id)
        .ok_or_else(|| Error::from("流程定义保存后回查失败"))
}

/// 发布（同 flowCode 下旧版本自动降级）
pub async fn publish(id: i64) -> Result<()> {
    service::engine()?.publish(id).await.map_err(flow_err)
}

/// 取消发布（已有实例时不允许）
pub async fn unpublish(id: i64) -> Result<()> {
    service::engine()?.un_publish(id).await.map_err(flow_err)
}

/// 复制为新版本
pub async fn copy(id: i64) -> Result<DefinitionVO> {
    let def = service::engine()?.copy_def(id).await.map_err(flow_err)?;
    detail(def.id.unwrap_or_default()).await
}

/// 删除（存在实例时引擎会拒绝）
pub async fn remove(ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        return Err(Error::from("请选择要删除的流程定义"));
    }
    service::engine()?.remove_def(ids).await.map_err(flow_err)
}

/// 挂起 / 激活
pub async fn set_activity(id: i64, active: bool) -> Result<()> {
    let engine = service::engine()?;
    if active {
        engine.active_def(id).await.map_err(flow_err)
    } else {
        engine.un_active_def(id).await.map_err(flow_err)
    }
}

/// 导入 JSON（总是生成新版本）
pub async fn import(json: &str) -> Result<DefinitionVO> {
    let def = service::engine()?.import_json(json).await.map_err(flow_err)?;
    let id = match def.id {
        Some(id) => id,
        None => latest_id(&def.flow_code).await?,
    };
    detail(id).await
}

/// 导出 JSON 字符串
pub async fn export(id: i64) -> Result<(String, String)> {
    let engine = service::engine()?;
    let def = engine.get_all_data_definition(id).await.map_err(flow_err)?;
    let json = engine.export_json(id).await.map_err(flow_err)?;
    let file_name = format!("{}_{}.json", def.flow_code, def.version);
    Ok((file_name, json))
}

/// 分类 id → 分类名（列表展示用）
async fn category_names() -> HashMap<String, String> {
    crate::modules::workflow::service::category::list(None)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|c| (c.category_id, c.category_name))
        .collect()
}

/// 详情接口只补一个分类名，从设计器共用的分类快照里取（避免再查一次库）
fn category_name_from_cache(id: &str) -> Option<String> {
    fn walk(nodes: &[warm_flow_core::dto::Tree], id: &str) -> Option<String> {
        for node in nodes {
            if node.id.as_deref() == Some(id) || node.value.as_deref() == Some(id) {
                return node.name.clone();
            }
            if let Some(found) = walk(&node.children, id) {
                return Some(found);
            }
        }
        None
    }
    walk(&ui::ui().categories(), id)
}
