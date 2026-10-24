use crate::modules::system::domain::vo::{RespJson, RespVO};
use crate::modules::system::service::CONTEXT;
use actix_web::{delete, get, web, Responder};

// 缓存监控信息（对应Spring GET /monitor/cache）
#[get("/cache")]
pub async fn get_info() -> impl Responder {
    let mut res = RespJson::success();
    let names = vec!["login_tokens", "sys_dict", "sys_config", "sys_menu"];
    let mut key_count = 0usize;
    for n in &names {
        key_count += CONTEXT.cache_service.keys(&format!("{}*", n)).await.unwrap_or_default().len();
    }
    // 前端 views/monitor/cache/index.vue 取 res.data.{info,dbSize,commandStats}，必须包 data 层
    // 注：完整 Redis INFO 需要给 cache_service 增加 INFO 命令支持，此处先返回可渲染结构
    // cacheBackend：当前缓存后端（moka/redis/mem）；cacheEntries：进程内条目数（redis/mem 恒为 0）
    res.insert(
        "data".to_string(),
        serde_json::json!({
            "cacheNames": names,
            "keyCount": key_count,
            "dbSize": key_count,
            "commandStats": [],
            "cacheBackend": CONTEXT.cache_service.backend(),
            "cacheEntries": CONTEXT.cache_service.entry_count().await,
            "info": {}
        }),
    );
    res.resp_json()
}

// 缓存名称列表（前端监控页使用）
#[get("/cache/getNames")]
pub async fn get_names() -> impl Responder {
    let names = vec!["login_tokens".to_string(), "sys_dict".to_string(), "sys_config".to_string(), "sys_menu".to_string()];
    RespVO::from_result(&Ok::<Vec<String>, crate::error::Error>(names)).resp_json()
}

// 缓存键名列表（前端监控页使用）
#[get("/cache/getKeys/{cache_name}")]
pub async fn get_keys(cache_name: web::Path<String>) -> impl Responder {
    let pattern = format!("{}*", cache_name.into_inner());
    let keys = CONTEXT.cache_service.keys(&pattern).await.unwrap_or_default();
    RespVO::from_result(&Ok::<Vec<String>, crate::error::Error>(keys)).resp_json()
}

// 缓存值（前端监控页使用）
#[get("/cache/getValue/{cache_name}/{cache_key}")]
pub async fn get_value(params: web::Path<(String, String)>) -> impl Responder {
    let (cache_name, cache_key) = params.into_inner();
    let key = if cache_key.starts_with(&cache_name) { cache_key } else { format!("{}:{}", cache_name, cache_key) };
    let v = CONTEXT.cache_service.get_string(&key).await.unwrap_or_default();
    RespVO::from_result(&Ok::<String, crate::error::Error>(v)).resp_json()
}

// 清理指定缓存名（前端监控页使用）
#[delete("/cache/clearCacheName/{cache_name}")]
pub async fn clear_cache_name(cache_name: web::Path<String>) -> impl Responder {
    let pattern = format!("{}*", cache_name.into_inner());
    let keys = CONTEXT.cache_service.keys(&pattern).await.unwrap_or_default();
    for k in keys {
        CONTEXT.cache_service.del(&k).await;
    }
    RespVO::<String>::from_success_info("清理成功！").resp_json()
}

// 清理指定缓存键（前端监控页使用）
#[delete("/cache/clearCacheKey/{cache_name}/{cache_key}")]
pub async fn clear_cache_key(params: web::Path<(String, String)>) -> impl Responder {
    let (cache_name, cache_key) = params.into_inner();
    let key = if cache_key.starts_with(&cache_name) { cache_key } else { format!("{}:{}", cache_name, cache_key) };
    CONTEXT.cache_service.del(&key).await;
    RespVO::<String>::from_success_info("清理成功！").resp_json()
}

// 清理全部缓存（前端监控页使用）
#[delete("/cache/clearCacheAll")]
pub async fn clear_cache_all() -> impl Responder {
    for prefix in ["login_tokens", "sys_dict", "sys_config", "sys_menu"] {
        let keys = CONTEXT.cache_service.keys(&format!("{}*", prefix)).await.unwrap_or_default();
        for k in keys {
            CONTEXT.cache_service.del(&k).await;
        }
    }
    RespVO::<String>::from_success_info("清理成功！").resp_json()
}
