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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use once_cell::sync::OnceCell;
use sea_orm_adapter::SeaOrmAdapter;
use sea_orm::{ConnectionTrait, Statement, DbBackend, FromQueryResult};
use tokio::sync::RwLock;

use casbin::prelude::*;
use casbin::{DefaultModel, Enforcer, Error};

/// 多租户/多应用域，暂统一为 `system`。
pub const DOMAIN: &str = "system";

static ENFORCER: OnceCell<Arc<RwLock<Enforcer>>> = OnceCell::new();

/// 「策略表非空」标志。
///
/// `Enforcer::get_policy()` 会把整张策略表克隆成一个 `Vec<Vec<String>>`
/// （每条策略 4 个 String），在每请求鉴权路径上调用会造成可观的堆分配。
/// 策略只在启动期由 [`bootstrap_policies`] 写入、此后无运行时增删，
/// 因此把「是否为空」缓存成原子标志，鉴权时零分配读取。
static POLICY_NON_EMPTY: AtomicBool = AtomicBool::new(false);

/// 依据当前 enforcer 实际策略刷新 [`POLICY_NON_EMPTY`]。
///
/// 任何会改变策略集合的操作（播种、新增/删除策略、save_policy）之后都应调用，
/// 否则缓存标志会与实际策略脱节。
pub fn refresh_policy_flag(e: &Enforcer) {
    POLICY_NON_EMPTY.store(!e.get_policy().is_empty(), Ordering::Relaxed);
    // 策略变了，历史鉴权结果全部失效
    clear_decision_cache();
}

// ---------------------------------------------------------------------------
// 鉴权结果缓存
// ---------------------------------------------------------------------------
//
// `Enforcer::enforce` 要走一遍 matcher：遍历全部 `p` 策略，逐条执行
// `keyMatch2`（内部要编译/匹配正则）。策略数量约为「角色数 × 菜单数」，
// 每请求一趟并不便宜。而同一用户在短时间内重复访问同一 (path, method)
// 的结果是稳定的，因此做一层短 TTL 的判定缓存。
//
// 一致性：角色/菜单变更会重新播种策略（`refresh_policy_flag`）或手动清缓存，
// 且 TTL 只有 60s，最坏情况下的权限延迟也可接受。

type DecisionKey = (String, String, String);

/// (user, path, method) -> (是否允许, 写入时刻)
static DECISION_CACHE: once_cell::sync::Lazy<Mutex<HashMap<DecisionKey, (bool, Instant)>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

const DECISION_TTL: Duration = Duration::from_secs(60);
const DECISION_CACHE_MAX: usize = 20_000;

/// 清空鉴权结果缓存（策略变更 / 手动刷新权限后调用）。
pub fn clear_decision_cache() {
    let mut g = DECISION_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    g.clear();
}

fn decision_get(key: &DecisionKey) -> Option<bool> {
    let g = DECISION_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    match g.get(key) {
        Some((allowed, at)) if at.elapsed() < DECISION_TTL => Some(*allowed),
        _ => None,
    }
}

fn decision_put(key: DecisionKey, allowed: bool) {
    let mut g = DECISION_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() >= DECISION_CACHE_MAX {
        // 粗粒度容量保护：满了就整体清空（写少读多，极少触发）
        g.clear();
    }
    g.insert(key, (allowed, Instant::now()));
}

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
            // casbin_rule 可能已存在历史策略，据此初始化「策略非空」标志
            refresh_policy_flag(&e);
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
    //
    // 这里读原子标志而非 `e.get_policy().is_empty()`：后者每次调用都会克隆整张
    // 策略表（Vec<Vec<String>>），在每请求路径上是纯粹的浪费。
    if !POLICY_NON_EMPTY.load(Ordering::Relaxed) {
        log::warn!("[ruoyi_rust] casbin policy empty, deny by default");
        return Ok(false);
    }
    let method_uc = method.to_uppercase();
    // 先查判定缓存：命中直接返回，省掉 enforcer 的锁 + matcher 遍历
    let key: DecisionKey = (user.to_string(), path.to_string(), method_uc.clone());
    if let Some(allowed) = decision_get(&key) {
        return Ok(allowed);
    }
    let allowed = e.enforce(vec![
        user.to_string(),
        DOMAIN.to_string(),
        path.to_string(),
        method_uc,
    ])?;
    decision_put(key, allowed);
    Ok(allowed)
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
    // 播种完成，刷新「策略非空」标志，使鉴权路径从此真正生效
    refresh_policy_flag(&e);
    log::info!("[ruoyi_rust] casbin policies bootstrapped from role/menu tables");
}
