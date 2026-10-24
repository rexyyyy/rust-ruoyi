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


///Whether the interface is in the whitelist
/// 匹配优先级：相等 > 原 contains > 前缀（静态资源目录）> 后缀（兼容不带 base_api 的短写）
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x == path || x.contains(path) {
            return true;
        }
        // 前缀匹配：静态资源目录，如 /dev-api/profile/avatar/xxx.png
        // 注意 x=="/" 必须排除，否则所有 path 都以 "/" 开头会被误判为白名单
        if x.len() > 1 && path.starts_with(x) {
            return true;
        }
        // 后缀匹配：兼容配置里写成 "/login" 而实际 path 是 "/dev-api/login"
        if x.len() > 1 && path.len() > x.len() && path.ends_with(x) {
            return true;
        }
    }
    //dbg!("path {}",&path);
    return false;
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
