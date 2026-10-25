//! 内置示例流程（从 RuoYi-Vue-Plus Java 版移植）。
//!
//! `leave1.json` 原样取自 Java 工程 `script/leave/leave1.json`：
//! 开始 → 申请人 → 组长(`role:1`) → 部门主管(`role:3@@role:4`) → 结束。
//!
//! 首次启动时如果库里还没有 `leave1` 的已发布定义，就自动导入并发布，
//! 这样「工作流」菜单一进去就能直接发起请假，不需要先手工设计。

use std::sync::Arc;

use warm_flow_core::engine::FlowEngine;
use warm_flow_core::store::DefinitionQuery;

use crate::error::{Error, Result};
use crate::modules::workflow::state;

/// 与 `service::leave::FLOW_CODE` 对应
pub const FLOW_CODE: &str = "leave1";

/// Java 工程导出的流程 JSON（保留原始 nodeType 字符串形态，顺带验证宽松反序列化）
const LEAVE1_JSON: &str = include_str!("../../../../resources/workflow/leave1.json");

/// 确保示例流程已发布；已存在则直接跳过。
pub async fn ensure_demo_flow() -> Result<()> {
    let Some(engine) = state::engine() else {
        return Err(Error::from("工作流引擎未初始化"));
    };
    let published = engine
        .list_definitions(&DefinitionQuery {
            flow_code: Some(FLOW_CODE.to_string()),
            is_publish: Some(1),
            page_num: 1,
            page_size: 1,
            ..Default::default()
        })
        .await
        .map_err(|e| Error::from(e.to_string()))?;
    if !published.rows.is_empty() {
        return Ok(());
    }

    let imported = engine
        .import_json(LEAVE1_JSON)
        .await
        .map_err(|e| Error::from(format!("导入示例流程 leave1 失败: {e}")))?;
    // 引擎的 save_def 新建分支不会回填主键，这里按流程编码回查最新一条兜底
    let id = match imported.id {
        Some(id) => id,
        None => latest_definition_id(engine.clone(), FLOW_CODE).await?,
    };
    engine
        .publish(id)
        .await
        .map_err(|e| Error::from(format!("发布示例流程 leave1 失败: {e}")))?;
    log::info!("[workflow] 已导入并发布示例流程 leave1（id = {id}）");
    Ok(())
}

/// 按流程编码取最新一条定义的 id（版本号最大者）
async fn latest_definition_id(engine: Arc<FlowEngine>, flow_code: &str) -> Result<i64> {
    let page = engine
        .list_definitions(&DefinitionQuery {
            flow_code: Some(flow_code.to_string()),
            page_num: 1,
            page_size: 10,
            ..Default::default()
        })
        .await
        .map_err(|e| Error::from(e.to_string()))?;
    page.rows
        .iter()
        .max_by(|a, b| a.version.cmp(&b.version))
        .and_then(|d| d.id)
        .ok_or_else(|| Error::from("导入示例流程 leave1 后未拿到定义 id"))
}
