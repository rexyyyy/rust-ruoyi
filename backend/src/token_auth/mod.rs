use actix_web::{HttpRequest};
use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use crate::modules::system::domain::vo::{JWTToken, UserCache};
use crate::error::Error;
use crate::modules::system::service::CONTEXT;

pub const TOKEN_PREFIX: &'static str = "Bearer ";

pub fn get_token(req: &HttpRequest) -> String {
    let mut token = req
        .headers()
        .get("Authorization")
        .map(|v| v.to_str().unwrap_or_default().to_string())
        .unwrap_or_default();
    if let Some(stripped) = token.strip_prefix(TOKEN_PREFIX) {
        token = stripped.to_string();
    }
    token
}


/// 尝试剥掉 `base_api` 前缀。
///
/// - 返回 `Some(归一化后的路径)`：该路径确实挂在 `base_api` 之下；
/// - 返回 `None`：该路径**不属于** `base_api`（例如 `/auth/login`、`/evil/login`）。
///
/// 只认完整路径段边界：`/dev-apiXYZ` 不算挂在 `/dev-api` 之下。
fn strip_api<'a>(path: &'a str, base_api: &str) -> Option<&'a str> {
    if base_api.is_empty() || base_api == "/" {
        return None;
    }
    match path.strip_prefix(base_api) {
        Some(rest) if rest.is_empty() => Some("/"),
        Some(rest) if rest.starts_with('/') => Some(rest),
        _ => None,
    }
}

/// 判断请求路径是否命中白名单中的某一项。
///
/// 匹配规则（均为路径段安全的精确语义）：
/// 1. `base_api` 归一化后**精确相等**；
/// 2. 归一化后命中**目录型前缀**且位于路径段边界（静态资源，如 `profile/avatar/xxx.png`）。
///
/// 明确**移除**了旧实现的两种不安全匹配：
/// - `entry.contains(path)`（反向子串）：请求路径只要碰巧是白名单项的任意子串即放行；
/// - `path.ends_with(entry)`（任意后缀）：`/any/thing/login` 会被当成 `/login` 放行。
fn white_list_match(path: &str, entry: &str, base_api: &str) -> bool {
    if entry.is_empty() || entry == "/" {
        return false;
    }
    let p_norm = strip_api(path, base_api);
    let e_norm = strip_api(entry, base_api);

    // 请求路径不在 base_api 之下时，禁止匹配「原本挂在 base_api 下」的白名单项，
    // 否则 `/auth/login` 会被当作 `/dev-api/auth/login` 放行。
    if p_norm.is_none() && e_norm.is_some() {
        return false;
    }

    let p = p_norm.unwrap_or(path);
    let e = e_norm.unwrap_or(entry);

    // 1) 归一化后精确相等
    if p == e {
        return true;
    }
    // 2) 目录前缀 + 段边界（防止 /avatarXXX 被当成 /avatar）
    match p.strip_prefix(e) {
        Some(tail) => tail.starts_with('/'),
        None => false,
    }
}

///Whether the interface is in the whitelist
///
/// 判断请求路径是否在白名单内。
///
/// 契约：`path` 为请求的完整路径（含 `base_api`）。仅当其与白名单项在归一化为
/// 模块内路径后精确相等、或命中目录型白名单的段边界前缀时才放行。
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    let base_api = &CONTEXT.config.base_api;
    CONTEXT
        .config
        .white_list_api
        .iter()
        .any(|x| white_list_match(path, x, base_api))
}

///Check whether the token_auth is valid and has not expired
pub async fn checked_token(token: &str) -> Result<UserCache, Error> {
    //check token_auth alive
    let claims = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match claims {
        Ok(c) => {
            let key = format!("{}{}", LOGIN_TOKEN_KEY, c.login_user_key);
            // GET + EXPIRE 合并为单次 pipeline 往返（原来分两条命令，每次还各建一条连接）
            let user_cache: Result<UserCache, Error> = CONTEXT
                .cache_service
                .get_json_refresh_ttl(&key, (CONTEXT.config.token_expired_min * 60) as i32)
                .await;
            user_cache
        }
        Err(e) => {
            return Err(crate::error::Error::from(e.to_string()));
        }
    }
}

// 注：原 `check_auth` / `check_permit`（配合 permit-lib `#[has_permit]` 宏的注解式鉴权）
// 已随 permit-lib 一并移除——接口鉴权统一由 `middleware::auth_actix::Auth` 中间件
// 做「token 校验 + casbin(path, method) 授权」，见该文件。
// 若日后需要按权限字符串（sys:user:list 这类）做接口级校验，推荐实现一个
// actix 提取器（如 `Permit<...>`），无需过程宏。

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "/dev-api";

    /// 回归用例：旧实现 `entry.contains(path)` 会放行任意「白名单项的子串」。
    #[test]
    fn rejects_substring_of_whitelist_entry() {
        // `/auth` 是 `/dev-api/auth/login` 的子串，旧实现返回 true（绕过鉴权）
        assert!(!white_list_match("/dev-api/auth", "/dev-api/auth/login", BASE));
        assert!(!white_list_match("/auth/login", "/dev-api/auth/login", BASE));
    }

    /// 回归用例：旧实现 `path.ends_with(entry)` 会放行任意「以白名单项结尾」的路径。
    #[test]
    fn rejects_arbitrary_prefix_with_same_suffix() {
        // 任意前缀拼接 `/login` 都应被拒绝
        assert!(!white_list_match("/evil/login", "/login", BASE));
        assert!(!white_list_match("/system/user/login", "/dev-api/login", BASE));
        assert!(!white_list_match("/evil/logout", "/logout", BASE));
    }

    /// 目录型前缀必须落在路径段边界，`/avatarXXX` 不能被当成 `/avatar`。
    #[test]
    fn rejects_prefix_without_segment_boundary() {
        assert!(!white_list_match(
            "/dev-api/profile/avatarX/1.png",
            "/dev-api/profile/avatar",
            BASE
        ));
        assert!(!white_list_match(
            "/dev-api/profile/avatarEvil",
            "/dev-api/profile/avatar",
            BASE
        ));
    }

    /// 合法场景必须仍然放行（含两种配置风格）。
    #[test]
    fn accepts_legitimate_paths() {
        // 精确命中
        assert!(white_list_match(
            "/dev-api/auth/login",
            "/dev-api/auth/login",
            BASE
        ));
        // application.yml 风格：配置项写成不带前缀的 /login，实际路径带 /dev-api
        assert!(white_list_match("/dev-api/login", "/login", BASE));
        assert!(white_list_match("/dev-api/logout", "/logout", BASE));
        // 完整写法命中
        assert!(white_list_match("/dev-api/login", "/dev-api/login", BASE));
        // 静态资源目录
        assert!(white_list_match(
            "/dev-api/profile/avatar/2026/01/01/a.png",
            "/dev-api/profile/avatar",
            BASE
        ));
    }

    /// 非白名单的普通业务接口必须拒绝。
    #[test]
    fn rejects_normal_business_paths() {
        assert!(!white_list_match(
            "/dev-api/system/user/list",
            "/dev-api/auth/login",
            BASE
        ));
        assert!(!white_list_match(
            "/dev-api/monitor/online/list",
            "/dev-api/login",
            BASE
        ));
    }

    /// base_api 为空（部分部署直接挂根路径）时退化为纯精确匹配。
    #[test]
    fn works_with_empty_base_api() {
        assert!(white_list_match("/login", "/login", ""));
        assert!(white_list_match("/captchaImage", "/captchaImage", ""));
        assert!(!white_list_match("/evil/login", "/login", ""));
    }
}
