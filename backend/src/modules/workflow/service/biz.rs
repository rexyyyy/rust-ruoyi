//! 流程状态 → 业务状态回写。
//!
//! Java 版通过 warm-flow 的事件监听器（`FlowProcessEventHandler`）在流程流转后
//! 更新业务表；Rust 侧监听器是纯内存回调，拿不到数据库句柄，因此改为
//! **service 在动作成功后显式调用** [`sync_business_status`]：
//!
//! ```text
//! task::start / task::complete / task::termination / task::revoke
//!         └── sync_after_action(instance_id)
//!                 └── biz::sync_business_status(&instance)
//!                         └── UPDATE test_leave SET status = ...
//! ```
//!
//! 本模块目前只认识请假单（`test_leave`）；接入新的业务表时在这里按
//! `flow_code` 或业务主键前缀分流即可。

use sea_orm::{ConnectionTrait, Value};
use warm_flow_core::enums::FlowStatus;
use warm_flow_core::entity::Instance;

use crate::db::raw_stmt;
use crate::error::{Error, Result};

/// 业务状态取值，与 `sys_dict_data` 里 `wf_business_status` 字典保持一致。
///
/// 引擎内部的 `flow_status` 是纯数字（0/1/…/13），业务表与前端展示用的是
/// 这些语义化字符串，两者在这里做一次映射。
pub mod status {
    pub const DRAFT: &str = "draft";
    pub const WAITING: &str = "waiting";
    pub const FINISH: &str = "finish";
    pub const CANCEL: &str = "cancel";
    pub const BACK: &str = "back";
    pub const TERMINATION: &str = "termination";
    pub const INVALID: &str = "invalid";
}

/// `flow_status` → 业务状态
pub fn flow_status_to_business(flow_status: &str) -> &'static str {
    match flow_status {
        FlowStatus::TO_BE_SUBMIT | FlowStatus::PENDING => status::DRAFT,
        FlowStatus::FINISHED => status::FINISH,
        FlowStatus::TERMINATE => status::TERMINATION,
        FlowStatus::NULLIFY => status::INVALID,
        FlowStatus::INVALID => status::INVALID,
        FlowStatus::CANCEL => status::CANCEL,
        FlowStatus::REJECT => status::BACK,
        // 1 审批中 / 2 通过 / 3 自动通过 / 7 取回 / 11 任务退回 / 12 重新发起
        _ => status::WAITING,
    }
}

/// 重复发起校验（对应 Java `BusinessStatusEnum.checkStartStatus`）。
///
/// 只有「未提交 / 已撤销 / 已退回 / 暂存 / 已作废」的实例允许再次发起；
/// 流程还在审批中或已结束时，重复发起会产生两条并行实例。
pub fn check_start_status(flow_status: &str) -> Result<()> {
    match flow_status {
        FlowStatus::TO_BE_SUBMIT
        | FlowStatus::CANCEL
        | FlowStatus::REJECT
        | FlowStatus::PENDING
        | FlowStatus::INVALID => Ok(()),
        _ => Err(Error::from("当前流程已启动，请勿重复发起")),
    }
}

/// 业务编号：优先取流程变量里的 `businessCode`，否则退回业务主键。
pub fn business_code(instance: &Instance) -> Option<String> {
    let var = instance.variable_map();
    var.get("businessCode")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| Some(instance.business_id.clone()))
}

/// 把实例的最新状态写回业务表。失败只记日志，不影响流程流转本身。
pub async fn sync_business_status(instance: &Instance) {
    let Some(db) = crate::db::db() else {
        return;
    };
    // 目前只有请假单（主键为数值型）接了工作流；其它业务（主键非数字）跳过
    let Ok(leave_id) = instance.business_id.trim().parse::<i64>() else {
        return;
    };
    let status = flow_status_to_business(&instance.flow_status);
    let now = chrono::Local::now().naive_local();
    let stmt = raw_stmt(
        "UPDATE test_leave SET status = $1, update_time = $2 WHERE id = $3",
        vec![
            Value::from(status.to_string()),
            Value::from(now),
            Value::from(leave_id),
        ],
    );
    if let Err(e) = db.execute(stmt).await {
        log::warn!("[workflow] 回写请假单 {leave_id} 状态失败: {e}");
    }
}

/// 回填业务编号（流程开启后把 `applyCode` 补到业务表）
pub async fn fill_business_code(business_id: &str, code: &str) {
    let Some(db) = crate::db::db() else {
        return;
    };
    let Ok(leave_id) = business_id.trim().parse::<i64>() else {
        return;
    };
    let stmt = raw_stmt(
        "UPDATE test_leave SET apply_code = $1 WHERE id = $2",
        vec![Value::from(code.to_string()), Value::from(leave_id)],
    );
    if let Err(e) = db.execute(stmt).await {
        log::warn!("[workflow] 回填请假单 {leave_id} 业务编号失败: {e}");
    }
}
