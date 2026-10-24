use crate::modules::system::domain::vo:: {RespJson, RespVO, SysUserOnlineVO, UserCache};
use crate::modules::system::service::CONTEXT;
use actix_web::{get, delete, web, Responder};
use crate::config::cache_variables::LOGIN_TOKEN_KEY;
use actix_web::HttpRequest;

#[get("/online/list")]
pub async fn page() -> impl Responder {
    // keys 内部走 SCAN（非阻塞）；再一次性 MGET，避免逐个 GET 的 N+1 往返
    let keys = CONTEXT.cache_service.keys(&format!("{}*", LOGIN_TOKEN_KEY)).await.unwrap_or_default();
    let values = CONTEXT.cache_service.mget(keys.clone()).await.unwrap_or_default();

    let mut user_online_list = vec![];

    for (k, v) in keys.iter().zip(values.into_iter()) {
        let _ = k;
        if v.is_empty() {
            continue;
        }
        match serde_json::from_str::<UserCache>(&v) {
            Ok(u) => {
                let user_online = SysUserOnlineVO {
                    token_id: Some(u.token_key.trim_start_matches(LOGIN_TOKEN_KEY).to_string()),
                    dept_name: None,
                    user_name: Some(u.user_name),
                    ipaddr: None,
                    login_location: None,
                    phonenumber: None,
                    browser: None,
                    os: None,
                    login_time: Some(u.login_time),
                };
                user_online_list.push(user_online);
            }
            Err(_) => {}
        }
    }
    let mut res = RespJson::success();
    res.insert("rows".to_string(), serde_json::json!(user_online_list));
    res.insert("total".to_string(), serde_json::json!(user_online_list.len()));
    res.resp_json()
}

#[delete("/online/{token_id}")]
pub async fn force_logout(token_id: web::Path<String>) -> impl Responder {
    let res = CONTEXT.cache_service.del(&format!("{}{}", LOGIN_TOKEN_KEY, token_id.into_inner())).await.unwrap();
    if res {
        RespVO::<u64>::from_success_info("强制成功！").resp_json()
    } else {
        RespVO::<u64>::from_success_info("强制失败！").resp_json()
    }
}

// 获取当前在线用户信息（对应Spring GET /monitor/online）
#[get("/online")]
pub async fn get_online(req: HttpRequest) -> impl Responder {
    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await;
    match user_cache {
        Ok(u) => RespVO::from_result(&Ok(u)).resp_json(),
        Err(e) => RespVO::from_error_result(500, &Err::<UserCache, crate::error::Error>(e)).resp_json(),
    }
}

// 强退当前用户自身（对应Spring DELETE /monitor/online/myself/{tokenId}）
#[delete("/online/myself/{token_id}")]
pub async fn remove_self(token_id: web::Path<String>) -> impl Responder {
    CONTEXT.cache_service.del(&format!("{}{}", LOGIN_TOKEN_KEY, token_id.into_inner())).await;
    RespVO::<String>::from_success_info("操作成功").resp_json()
}