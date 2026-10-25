//! 流程实例。对标 Java `FlwInstanceServiceImpl`。
//!
//! 实例是「一张单据跑一次流程」的记录，`business_id` 指回业务主键，
//! 流程图（节点/连线三原色）由引擎根据实例上保存的 `defJson` 还原。

use std::collections::HashMap;

use warm_flow_core::dto::DefJson;
use warm_flow_core::entity::Definition;
use warm_flow_core::store::InstanceQuery;
use warm_flow_web::UiExtension;

use crate::error::{Error, Result};
use crate::modules::workflow::domain::bo::InstancePageBo;
use crate::modules::workflow::domain::vo::InstanceVO;
use crate::modules::workflow::domain::Page;
use crate::modules::workflow::service::{self, flow_err};

/// 实例分页
pub async fn page(bo: &InstancePageBo) -> Result<Page<InstanceVO>> {
    let engine = service::engine()?;
    let page_no = bo.page_no.unwrap_or(1).max(1);
    let page_size = bo.page_size.unwrap_or(10).clamp(1, 200);

    // flow_code 可能对应多个历史版本；单版本时直接下推到 SQL，多版本时先按版本过滤
    let mut definition_filter: Option<i64> = None;
    let mut code_ids: Vec<i64> = Vec::new();
    if let Some(code) = service::blank_to_none(bo.flow_code.as_deref()) {
        code_ids = service::definition_ids_by_code(&code).await;
        if code_ids.len() == 1 {
            definition_filter = code_ids.first().copied();
            code_ids.clear();
        }
    }

    let query = InstanceQuery {
        definition_id: definition_filter,
        business_id: service::blank_to_none(bo.business_id.as_deref()),
        flow_status: service::blank_to_none(bo.flow_status.as_deref()),
        create_by: service::blank_to_none(bo.create_by.as_deref()),
        page_num: page_no,
        page_size,
    };
    let mut data = engine.list_instances(&query).await.map_err(flow_err)?;
    if !code_ids.is_empty() {
        data.rows.retain(|ins| code_ids.contains(&ins.definition_id));
        data.total = data.rows.len() as u64;
    }

    let defs = definition_map(data.rows.iter().map(|i| i.definition_id)).await;
    let rows = data
        .rows
        .into_iter()
        .map(|ins| fill(&mut InstanceVO::from(ins), &defs))
        .collect::<Vec<_>>();
    Ok(Page::new(fill_creators(rows).await, data.total, page_no, page_size))
}

/// 实例详情
pub async fn detail(id: i64) -> Result<InstanceVO> {
    let engine = service::engine()?;
    let instance = engine
        .store()
        .get_instance(id)
        .await
        .map_err(flow_err)?
        .ok_or_else(|| Error::from(format!("流程实例 {id} 不存在")))?;
    let defs = definition_map([instance.definition_id]).await;
    let mut vo = fill(&mut InstanceVO::from(instance), &defs);
    if let Some(id) = vo.create_by.clone() {
        vo.create_by_name = service::user_names([id.as_str()]).await.remove(&id);
    }
    Ok(vo)
}

/// 给实例列表补发起人昵称（引擎里 `create_by` 存的是用户 id，列表要给人看名字）
async fn fill_creators(mut rows: Vec<InstanceVO>) -> Vec<InstanceVO> {
    let names = service::user_names(rows.iter().filter_map(|r| r.create_by.as_deref())).await;
    if names.is_empty() {
        return rows;
    }
    for r in rows.iter_mut() {
        if let Some(name) = r.create_by.as_deref().and_then(|id| names.get(id)) {
            r.create_by_name = Some(name.clone());
        }
    }
    rows
}

/// 流程图（节点/连线的办理状态都在这里）
pub async fn chart(instance_id: i64) -> Result<DefJson> {
    let engine = service::engine()?;
    let mut def_json = engine.query_flow_chart(instance_id).await.map_err(flow_err)?;
    def_json.chart_status_color = Some(engine.chart_rgb(&def_json.model()));
    def_json.top_text_show = Some(engine.config().top_text_show);
    crate::modules::workflow::ui::ui().chart_ext(&mut def_json);
    Ok(def_json)
}

/// 实例的流程变量
pub async fn variables(instance_id: i64) -> Result<serde_json::Map<String, serde_json::Value>> {
    let engine = service::engine()?;
    let instance = engine
        .store()
        .get_instance(instance_id)
        .await
        .map_err(flow_err)?
        .ok_or_else(|| Error::from(format!("流程实例 {instance_id} 不存在")))?;
    Ok(instance.variable_map())
}

/// 激活 / 挂起实例
pub async fn set_activity(id: i64, active: bool) -> Result<()> {
    let engine = service::engine()?;
    if active {
        engine.active_instance(id).await.map_err(flow_err)
    } else {
        engine.un_active_instance(id).await.map_err(flow_err)
    }
}

/// 删除实例（连同待办 / 历史 / 办理人）
pub async fn remove(ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        return Err(Error::from("请选择要删除的流程实例"));
    }
    service::engine()?.remove_instance(ids).await.map_err(flow_err)
}

/// 按业务主键查实例（业务侧判断「是否已发起」）
pub async fn find_by_business(business_id: &str) -> Result<Option<InstanceVO>> {
    let engine = service::engine()?;
    let data = engine
        .list_instances(&InstanceQuery {
            business_id: Some(business_id.to_string()),
            page_num: 1,
            page_size: 1,
            ..Default::default()
        })
        .await
        .map_err(flow_err)?;
    let Some(instance) = data.rows.into_iter().next() else {
        return Ok(None);
    };
    let defs = definition_map([instance.definition_id]).await;
    Ok(Some(fill(&mut InstanceVO::from(instance), &defs)))
}

// ----------------------------------------------------------------------
// 内部辅助
// ----------------------------------------------------------------------

/// 批量取流程定义，用于补 flowCode / flowName
async fn definition_map(ids: impl IntoIterator<Item = i64>) -> HashMap<i64, Definition> {
    let mut out = HashMap::new();
    let Some(engine) = crate::modules::workflow::state::engine() else {
        return out;
    };
    for id in ids {
        if out.contains_key(&id) {
            continue;
        }
        if let Ok(Some(def)) = engine.store().get_definition(id).await {
            out.insert(id, def);
        }
    }
    out
}

fn fill(vo: &mut InstanceVO, defs: &HashMap<i64, Definition>) -> InstanceVO {
    let def_id = vo.definition_id.parse::<i64>().unwrap_or_default();
    if let Some(def) = defs.get(&def_id) {
        vo.flow_name = Some(def.flow_name.clone());
        vo.flow_code = Some(def.flow_code.clone());
    }
    vo.clone()
}
