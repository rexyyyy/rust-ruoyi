//! 工作流业务层。
//!
//! 对标 Java 版 `ruoyi-workflow` 的 `service/impl`：
//! - [`category`]   —— 流程分类（flow_category）
//! - [`definition`] —— 流程定义（设计器数据、发布、导入导出）
//! - [`instance`]   —— 流程实例（查询、挂起、删除、流程图）
//! - [`task`]       —— 待办 / 已办 / 发起 / 办理 / 终止 / 转办
//! - [`leave`]      —— 请假单示例业务（test_leave），演示「业务表 + 流程引擎」
//! - [`biz`]        —— 流程状态回写业务表（替代 Java 的事件监听）
//! - [`demo`]       —— 内置示例流程（leave1）自动导入
//!
//! 与 Java 的差异：warm-flow 的监听器在 Rust 侧是纯内存回调，拿不到数据库事务，
//! 因此「流程状态 → 业务状态」不放在监听器里做，而是由 service 在动作完成后显式调用
//! [`biz::sync_business_status`]。

pub mod biz;
pub mod category;
pub mod definition;
pub mod demo;
pub mod instance;
pub mod leave;
pub mod task;

use std::collections::HashMap;
use std::sync::Arc;

use sea_orm::{ConnectionTrait, Value};
use warm_flow_core::engine::FlowEngine;
use warm_flow_core::enums::SkipType;

use crate::db::{raw_stmt, raw_stmt0};
use crate::error::{Error, Result};

/// 取全局流程引擎；未初始化时返回业务错误而不是 panic。
pub fn engine() -> Result<&'static Arc<FlowEngine>> {
    super::state::engine_checked()
}

/// 把引擎错误统一收敛到本项目 `Error`（前端只看到 msg）
pub fn flow_err<E: std::fmt::Display>(e: E) -> Error {
    Error::from(e.to_string())
}

/// 跳转类型规范化：只接受 PASS / REJECT，其余按 PASS 处理
pub fn normalize_skip_type(raw: Option<&str>) -> &'static str {
    match raw.map(str::trim) {
        Some(v) if SkipType::is_reject(v) => SkipType::REJECT,
        _ => SkipType::PASS,
    }
}

/// 空串归一
pub fn blank_to_none(raw: Option<&str>) -> Option<String> {
    raw.map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

/// 当前用户是否超级管理员（role_id = 1）。
///
/// 超管不是流程里的「办理人」，但需要能操作任意任务，否则演示与运维都会卡住。
pub async fn is_admin(uid: i64) -> bool {
    if uid == 1 {
        return true;
    }
    let Some(db) = crate::db::db() else {
        return false;
    };
    db.query_one(raw_stmt(
        "SELECT 1 AS hit FROM sys_user_role WHERE user_id = $1 AND role_id = 1",
        vec![Value::from(uid)],
    ))
    .await
    .ok()
    .flatten()
    .is_some()
}

/// 当前用户的权限标识：用户 id + `role:{id}` + `dept:{id}`。
///
/// 引擎 `check_auth` 拿它与任务的办理人列表求交集；角色/部门标识同时带上，
/// 便于任务按角色配置时也能命中（引擎侧已把角色展开成用户，双保险）。
pub async fn permission_flags(uid: i64) -> Vec<String> {
    let mut out = vec![uid.to_string()];
    let Some(db) = crate::db::db() else {
        return out;
    };
    if let Ok(rows) = db
        .query_all(raw_stmt(
            "SELECT role_id FROM sys_user_role WHERE user_id = $1",
            vec![Value::from(uid)],
        ))
        .await
    {
        for row in rows {
            if let Ok(role_id) = row.try_get::<i64>("", "role_id") {
                out.push(format!("role:{role_id}"));
            }
        }
    }
    if let Ok(Some(row)) = db
        .query_one(raw_stmt(
            "SELECT dept_id FROM sys_user WHERE user_id = $1",
            vec![Value::from(uid)],
        ))
        .await
    {
        if let Ok(dept_id) = row.try_get::<i64>("", "dept_id") {
            out.push(format!("dept:{dept_id}"));
        }
    }
    out
}

/// 当前用户所属部门 id（写入流程变量 `initiatorDeptId`，供设计器部门条件使用）
pub async fn user_dept(uid: i64) -> Option<i64> {
    let db = crate::db::db()?;
    db.query_one(raw_stmt(
        "SELECT dept_id FROM sys_user WHERE user_id = $1",
        vec![Value::from(uid)],
    ))
    .await
    .ok()
    .flatten()
    .and_then(|row| row.try_get::<i64>("", "dept_id").ok())
}

/// 批量把用户 id 翻译成昵称（已办「办理人」、实例「发起人」等列表列显示用）
///
/// 一次 IN 查询取回，不逐行查库；非数字 id 或查无此人的 key 不进入结果集，
/// 由前端 `approverName || approver` 回退成原始值。
pub async fn user_names(ids: impl IntoIterator<Item = impl AsRef<str>>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut uniq: Vec<i64> = Vec::new();
    for raw in ids {
        let Ok(uid) = raw.as_ref().trim().parse::<i64>() else {
            continue;
        };
        if !uniq.contains(&uid) {
            uniq.push(uid);
        }
    }
    if uniq.is_empty() {
        return out;
    }
    let Some(db) = crate::db::db() else {
        return out;
    };
    let holders = (1..=uniq.len()).map(|i| format!("${i}")).collect::<Vec<_>>().join(",");
    let sql = format!("SELECT user_id, nick_name FROM sys_user WHERE user_id IN ({holders})");
    let vals = uniq.into_iter().map(Value::from).collect();
    let rows = match db.query_all(raw_stmt(sql, vals)).await {
        Ok(rows) => rows,
        Err(e) => {
            log::warn!("[workflow] 查询用户昵称失败: {e}");
            return out;
        }
    };
    for row in rows {
        let Ok(uid) = row.try_get::<i64>("", "user_id") else {
            continue;
        };
        let Ok(name) = row.try_get::<Option<String>>("", "nick_name") else {
            continue;
        };
        if let Some(name) = name.filter(|v| !v.is_empty()) {
            out.insert(uid.to_string(), name);
        }
    }
    out
}

/// 按 flow_code 取该流程全部版本的 definitionId（用于实例/任务按流程编码过滤）
pub async fn definition_ids_by_code(flow_code: &str) -> Vec<i64> {
    let Some(db) = crate::db::db() else {
        return Vec::new();
    };
    db.query_all(raw_stmt(
        "SELECT id FROM flow_definition WHERE flow_code = $1",
        vec![Value::from(flow_code.to_string())],
    ))
    .await
    .map(|rows| rows.iter().filter_map(|r| r.try_get::<i64>("", "id").ok()).collect())
    .unwrap_or_default()
}

/// 流程定义是否存在（用于启动前给友好提示）
pub async fn definition_exists(flow_code: &str) -> bool {
    let Some(db) = crate::db::db() else {
        return false;
    };
    db.query_one(raw_stmt0(format!(
        "SELECT 1 AS hit FROM flow_definition WHERE flow_code = '{}' AND is_publish = 1 LIMIT 1",
        flow_code.replace('\'', "")
    )))
    .await
    .ok()
    .flatten()
    .is_some()
}

/// 查询单列 i64 列表（内部辅助）
pub async fn query_i64_column(sql: &str, vals: Vec<Value>) -> Vec<i64> {
    let Some(db) = crate::db::db() else {
        return Vec::new();
    };
    db.query_all(raw_stmt(sql, vals))
        .await
        .map(|rows| rows.iter().filter_map(|r| r.try_get::<i64>("", "").ok()).collect())
        .unwrap_or_default()
}
