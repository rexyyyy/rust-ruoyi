//! 办理人权限处理器：把设计器里配置的 `role:1` / `dept:101` 翻译成真实用户 id。
//!
//! 对应 Java 版 `WorkflowPermissionHandler`，但实现方式不同：
//! - Java 在 `convertPermissions` 里同步查数据库（MyBatis 无异步约束）；
//! - Rust 的 `PermissionHandler::convert_permissions` 是同步函数，而 SeaORM 只有异步 API，
//!   在 async 上下文里阻塞等查询极易把连接池打死。因此这里改成
//!   「后台定时把 角色/部门 → 用户 映射刷进内存 + 同步读缓存」：
//!   转换是纯内存操作，30s 一次的全量刷新代价可以忽略。
//!
//! 支持的写法（多个用 `@@` 分隔，由引擎负责拆分）：
//! - `role:1`     角色下的所有启用用户
//! - `dept:101`   直属该部门的启用用户
//! - `user:5`     指定用户
//! - `5`          纯数字按用户 id 处理
//! - 其他（`${var}` 已被引擎替换过、`spel` 等）原样返回，交给后续逻辑兜底

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;
use sea_orm::{ConnectionTrait, FromQueryResult};
use warm_flow_core::handler::PermissionHandler;

use crate::db::raw_stmt0;

/// 角色/部门 → 用户 id 的快照
#[derive(Default)]
struct Snapshot {
    /// "role:1" -> [用户 id]
    by_role: HashMap<String, Vec<String>>,
    /// "dept:101" -> [用户 id]
    by_dept: HashMap<String, Vec<String>>,
}

pub struct RuoYiPermissionHandler {
    snapshot: RwLock<Snapshot>,
}

impl Default for RuoYiPermissionHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, FromQueryResult)]
struct UserRoleRow {
    user_id: i64,
    role_id: i64,
}

#[derive(Debug, FromQueryResult)]
struct UserDeptRow {
    user_id: i64,
    dept_id: i64,
}

impl RuoYiPermissionHandler {
    pub fn new() -> Self {
        Self {
            snapshot: RwLock::new(Snapshot::default()),
        }
    }

    /// 全量刷新角色/部门 → 用户映射。
    ///
    /// 只取 `status='0'`（正常）且未逻辑删除的用户，与 RuoYi 登录口径一致。
    pub async fn refresh(&self) -> Result<(), String> {
        let db = crate::db::db().ok_or_else(|| "数据库未初始化".to_string())?;

        let role_rows = db
            .query_all(raw_stmt0(
                "SELECT ur.user_id AS user_id, ur.role_id AS role_id \
                 FROM sys_user_role ur JOIN sys_user u ON u.user_id = ur.user_id \
                 WHERE u.status = '0' AND (u.del_flag = '0' OR u.del_flag IS NULL)",
            ))
            .await
            .map_err(|e| format!("查询角色用户失败: {e}"))?;

        let dept_rows = db
            .query_all(raw_stmt0(
                "SELECT user_id AS user_id, dept_id AS dept_id FROM sys_user \
                 WHERE status = '0' AND (del_flag = '0' OR del_flag IS NULL) AND dept_id IS NOT NULL",
            ))
            .await
            .map_err(|e| format!("查询部门用户失败: {e}"))?;

        let mut next = Snapshot::default();
        for row in &role_rows {
            let r = UserRoleRow::from_query_result(row, "").map_err(|e| e.to_string())?;
            next.by_role
                .entry(format!("role:{}", r.role_id))
                .or_default()
                .push(r.user_id.to_string());
        }
        for row in &dept_rows {
            let r = UserDeptRow::from_query_result(row, "").map_err(|e| e.to_string())?;
            next.by_dept
                .entry(format!("dept:{}", r.dept_id))
                .or_default()
                .push(r.user_id.to_string());
        }

        // 去掉空映射，避免缓存里堆积无意义 key
        next.by_role.retain(|_, v| !v.is_empty());
        next.by_dept.retain(|_, v| !v.is_empty());

        let role_cnt = next.by_role.len();
        let dept_cnt = next.by_dept.len();
        *self.snapshot.write() = next;
        log::debug!(
            "[workflow] 办理人缓存已刷新: {} 个角色, {} 个部门",
            role_cnt,
            dept_cnt
        );
        Ok(())
    }

    /// 起一个后台任务按固定周期刷新缓存；刷新失败只记 warn，保留上一份快照。
    pub fn spawn_refresher(self: &Arc<Self>, interval: Duration) {
        let me = self.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            // 第一次 tick 立即触发，跳过（init 时已经刷过一次）
            ticker.tick().await;
            loop {
                ticker.tick().await;
                if let Err(e) = me.refresh().await {
                    log::warn!("[workflow] 办理人缓存刷新失败: {}", e);
                }
            }
        });
    }
}

impl PermissionHandler for RuoYiPermissionHandler {
    fn convert_permissions(&self, permissions: Vec<String>) -> Vec<String> {
        let snap = self.snapshot.read();
        let mut out: Vec<String> = Vec::with_capacity(permissions.len());
        for raw in permissions {
            let key = raw.trim();
            if key.is_empty() {
                continue;
            }
            let users: Vec<String> = if let Some(rest) = key.strip_prefix("user:") {
                vec![rest.trim().to_string()]
            } else if key.chars().all(|c| c.is_ascii_digit()) {
                vec![key.to_string()]
            } else if let Some(list) = snap.by_role.get(key) {
                list.clone()
            } else if let Some(list) = snap.by_dept.get(key) {
                list.clone()
            } else {
                // 未识别的表达式原样保留：交给引擎的权限求交逻辑，至少不会静默丢配置
                vec![key.to_string()]
            };
            for u in users {
                if !u.is_empty() && !out.contains(&u) {
                    out.push(u);
                }
            }
        }
        out
    }
}
