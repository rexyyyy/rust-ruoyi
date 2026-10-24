//! 应用全局状态（AppState）。
//!
//! 把 `DatabaseConnection` + redis 客户端 + `ApplicationConfig` 打包成一个
//! ownership 归 `OnceCell` 的 `AppState`，既作为进程级单例（供 `pool!()` 等
//! 遗留代码读取），又通过 `web::Data<AppState>` 注入 actix app state，
//! 与 sea-orm 官方 actix 示例保持一致。
//!
//! 新增模块（job / workflow 等）建议在 handler 里用
//! `data: web::Data<AppState>` 取出后，把 `&data.db` / `&data.redis` 透传给
//! service，而非继续依赖全局宏；既有 `pool!()` 调用则读取同一个 AppState，
//! 保持零改动兼容。

use crate::config::config::ApplicationConfig;
use once_cell::sync::OnceCell;
use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;

/// 应用状态：数据库连接 + redis 客户端 + 运行配置。
///
/// 三个字段都可廉价 Clone（内部均为 `Arc` / 连接池共享），因此 `AppState`
/// 派生 `Clone`，便于在 `HttpServer::new` 闭包里 `web::Data::new(state.clone())`
/// 注入（每个 worker 持有一份 `Arc` 包裹的同一实例，非新建连接）。
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: RedisClient,
    pub config: ApplicationConfig,
}

static APP_STATE: OnceCell<AppState> = OnceCell::new();

/// 初始化全局 AppState。重复调用返回 Err（与 OnceCell 语义一致，避免重复建池）。
pub fn init(
    db: DatabaseConnection,
    redis: RedisClient,
    config: ApplicationConfig,
) -> Result<(), String> {
    APP_STATE
        .set(AppState { db, redis, config })
        .map_err(|_| "app state already initialized".to_string())
}

/// 取全局 AppState。未初始化时返回 None。
pub fn state() -> Option<&'static AppState> {
    APP_STATE.get()
}

/// 取数据库连接（供 `pool!()` 等遗留代码）。
pub fn db() -> Option<&'static DatabaseConnection> {
    APP_STATE.get().map(|s| &s.db)
}

/// 取 redis 客户端。
pub fn redis() -> Option<&'static RedisClient> {
    APP_STATE.get().map(|s| &s.redis)
}

/// 取运行配置。
pub fn config() -> Option<&'static ApplicationConfig> {
    APP_STATE.get().map(|s| &s.config)
}

/// 优雅关闭：归还数据库连接。redis 客户端无需显式关闭（drop 即断开连接池）。
pub async fn close() {
    if let Some(s) = APP_STATE.get() {
        // close() 要拿所有权，这里 clone 一份（DatabaseConnection 内部是 sqlx Pool，clone 很廉价）
        let _ = s.db.clone().close().await;
    }
}
