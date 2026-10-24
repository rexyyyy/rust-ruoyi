//! 基于 casbin 的 RBAC-with-domains 权限层。
//!
//! - 模型：`resources/rbac_with_domains_model.conf`
//!   - `r = sub, dom, obj, act`
//!   - `p = sub, dom, obj, act`（策略：`p, role:<key>, system, /api/system/..., *`）
//!   - `g = _, _, _`（分组：`g, <user_name>, role:<key>, system`）
//!   - matcher：`g(r.sub, p.sub, r.dom) && r.dom == p.dom && keyMatch2(r.obj, p.obj) && (r.act == p.act || p.act == "*")`
//! - 适配器：sea-orm-adapter，策略持久化在 `casbin_rule` 表（`SeaOrmAdapter::new` 会自动建表）。
//! - 优雅降级：enforcer 未就绪（casbin_rule 不可用 / 连接失败）或策略为空时，
//!   权限中间件仅做 token 校验，不阻断业务（详见 [`crate::middleware::auth_actix`]）。

use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::sync::Arc;

use once_cell::sync::OnceCell;
use sea_orm_adapter::SeaOrmAdapter;
use sea_orm::{ConnectionTrait, Statement, DbBackend, FromQueryResult};
use tokio::sync::RwLock;

use casbin::prelude::*;
use casbin::{DefaultModel, Enforcer, Error};

/// 多租户/多应用域，暂统一为 `system`。
pub const DOMAIN: &str = "system";

static ENFORCER: OnceCell<Arc<RwLock<Enforcer>>> = OnceCell::new();

/// 加载模型 + sea-orm 适配器，构建全局 enforcer。
/// 返回 `true` 表示已就绪；任何失败都记 warn 并返回 `false`，调用方降级。
pub async fn init_enforcer(db: &sea_orm::DatabaseConnection) -> bool {
    let model = match DefaultModel::from_str(include_str!("../resources/rbac_with_domains_model.conf")).await
    {
        Ok(m) => m,
        Err(e) => {
            log::warn!("[ruoyi_rust] casbin model parse failed: {}", e);
            return false;
        }
    };
    let adapter = match SeaOrmAdapter::new(db.clone()).await {
        Ok(a) => a,
        Err(e) => {
            log::warn!(
                "[ruoyi_rust] casbin adapter init failed (casbin_rule unavailable?): {}",
                e
            );
            return false;
        }
    };
    match Enforcer::new(model, adapter).await {
        Ok(e) => {
            let _ = ENFORCER.set(Arc::new(RwLock::new(e)));
            log::info!("[ruoyi_rust] casbin enforcer ready");
            true
        }
        Err(e) => {
            log::warn!("[ruoyi_rust] casbin enforcer init failed: {}", e);
            false
        }
    }
}

/// enforcer 是否就绪（用于中间件判断是否启用细粒度鉴权）。
pub fn enforcer_ready() -> bool {
    ENFORCER.get().is_some()
}

/// 细粒度鉴权：`enforce(user, domain, path, method)`。
///
/// - `Ok(true)`  允许
/// - `Ok(false)` 拒绝
/// - `Err`       enforcer 未就绪（中间件应降级）
///
/// 过渡期保护：若 enforcer 中没有任何 `p` 策略（casbin_rule 为空），直接放行，
/// 避免把所有人锁在门外；一旦播种了策略，即进入真正的 RBAC 拦截。
pub async fn enforce(user: &str, path: &str, method: &str) -> casbin::Result<bool> {
    let g = ENFORCER.get().ok_or_else(|| {
        Error::from(io::Error::new(ErrorKind::Other, "casbin enforcer not initialized"))
    })?;
    let e = g.read().await;
    // fail-closed：无策略时不放行（除中间件对 admin 的显式放行外）。
    // 避免策略表被清空 / 迁移未完成时，所有登录用户越权访问。
    if e.get_policy().is_empty() {
        log::warn!("[ruoyi_rust] casbin policy empty, deny by default");
        return Ok(false);
    }
    let method_uc = method.to_uppercase();
    e.enforce(vec![
        user.to_string(),
        DOMAIN.to_string(),
        path.to_string(),
        method_uc,
    ])
}

// ---------------------------------------------------------------------------
// 过渡期策略播种：从现有角色 / 菜单 / 用户-角色表生成 casbin_rule。
// 仅在 enforcer 就绪且当前无策略时调用一次。数据迁移完成后此函数可删除。
// ---------------------------------------------------------------------------

#[derive(Debug, FromQueryResult)]
struct RoleRow {
    role_id: String,
    role_key: String,
}
#[derive(Debug, FromQueryResult)]
struct MenuRow {
    menu_id: String,
    perms: Option<String>,
}
#[derive(Debug, FromQueryResult)]
struct RoleMenuRow {
    role_id: String,
    menu_id: String,
}
#[derive(Debug, FromQueryResult)]
struct UserRoleRow {
    user_id: String,
    role_id: String,
}
#[derive(Debug, FromQueryResult)]
struct UserRow {
    user_id: String,
    user_name: String,
}

/// 把 `system:user:list` 这类 perms 还原为后端路由路径 `/api/system/user/list`。
fn perms_to_path(base_api: &str, perms: &str) -> String {
    let base = base_api.trim_end_matches('/');
    format!("{}/{}", base, perms.replace(':', "/"))
}

/// 从现有 RuoYi 权限表播种 casbin 策略（幂等：已存在策略则跳过）。
/// 失败只记日志，不影响启动。
pub async fn bootstrap_policies() {
    let g = match ENFORCER.get() {
        Some(e) => e,
        None => return,
    };
    {
        let e = g.read().await;
        if !e.get_policy().is_empty() {
            return; // 已有策略，不再重复播种
        }
    }

    use crate::pool;

    let roles: Vec<RoleRow> = match pool!()
        .query_all(crate::db::raw_stmt(
            "SELECT role_id, role_key FROM sys_role WHERE status = '0'",
            vec![],
        ))
        .await
    {
        Ok(rows) => rows
            .iter()
            .filter_map(|r| RoleRow::from_query_result(r, "").ok())
            .collect(),
        Err(e) => {
            log::warn!("[ruoyi_rust] casbin bootstrap: read sys_role failed: {}", e);
            return;
        }
    };
    if roles.is_empty() {
        return;
    }
    let menus: Vec<MenuRow> = pool!()
        .query_all(crate::db::raw_stmt(
            "SELECT menu_id, perms FROM sys_menu WHERE perms IS NOT NULL AND perms <> ''",
            vec![],
        ))
        .await
        .unwrap_or_default()
        .iter()
        .filter_map(|r| MenuRow::from_query_result(r, "").ok())
        .collect();
    let role_menus: Vec<RoleMenuRow> = pool!()
        .query_all(crate::db::raw_stmt(
            "SELECT role_id, menu_id FROM sys_role_menu",
            vec![],
        ))
        .await
        .unwrap_or_default()
        .iter()
        .filter_map(|r| RoleMenuRow::from_query_result(r, "").ok())
        .collect();
    let user_roles: Vec<UserRoleRow> = pool!()
        .query_all(crate::db::raw_stmt(
            "SELECT user_id, role_id FROM sys_user_role",
            vec![],
        ))
        .await
        .unwrap_or_default()
        .iter()
        .filter_map(|r| UserRoleRow::from_query_result(r, "").ok())
        .collect();
    let users: Vec<UserRow> = pool!()
        .query_all(crate::db::raw_stmt(
            "SELECT user_id, user_name FROM sys_user",
            vec![],
        ))
        .await
        .unwrap_or_default()
        .iter()
        .filter_map(|r| UserRow::from_query_result(r, "").ok())
        .collect();

    let base = crate::modules::system::service::CONTEXT.config.base_api.clone();
    let role_key_by_id: HashMap<String, String> =
        roles.iter().map(|r| (r.role_id.clone(), r.role_key.clone())).collect();
    let perms_by_menu: HashMap<String, String> = menus
        .into_iter()
        .filter_map(|m| m.perms.map(|p| (m.menu_id, p)))
        .collect();
    let user_name_by_id: HashMap<String, String> =
        users.iter().map(|u| (u.user_id.clone(), u.user_name.clone())).collect();

    let mut e = g.write().await;
    for rm in &role_menus {
        if let (Some(rk), Some(perms)) =
            (role_key_by_id.get(&rm.role_id), perms_by_menu.get(&rm.menu_id))
        {
            let path = perms_to_path(&base, perms);
            let _ = e
                .add_policy(vec![
                    format!("role:{}", rk),
                    DOMAIN.to_string(),
                    path,
                    "*".to_string(),
                ])
                .await;
        }
    }
    for ur in &user_roles {
        if let (Some(un), Some(rk)) = (
            user_name_by_id.get(&ur.user_id),
            role_key_by_id.get(&ur.role_id),
        ) {
            let _ = e
                .add_grouping_policy(vec![un.clone(), format!("role:{}", rk), DOMAIN.to_string()])
                .await;
        }
    }
    if let Err(e2) = e.save_policy().await {
        log::warn!("[ruoyi_rust] casbin bootstrap: save_policy failed: {}", e2);
        return;
    }
    log::info!("[ruoyi_rust] casbin policies bootstrapped from role/menu tables");
}
