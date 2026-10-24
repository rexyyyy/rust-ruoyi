//! sea-orm 连接池。
//!
//! 与 `backend::db` 的做法保持一致（`ConnectOptions` + `OnceCell` + `ping()` fail-fast），
//! 这样 codegen 单独跑（CLI）和被 backend 内嵌调用时行为一致。
//!
//! 注意：被 backend 内嵌使用时，**不需要**调用 [`init_db`]，由 [`init_external`]
//! 直接复用宿主进程已有的连接池，避免同一个进程建两个池。

use std::time::Duration;

use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

const MAX_CONNECTIONS: u32 = 8;
const MIN_CONNECTIONS: u32 = 1;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(10);
const IDLE_TIMEOUT: Duration = Duration::from_secs(600);
const MAX_LIFETIME: Duration = Duration::from_secs(1800);

/// 自建连接池（CLI 场景）。传入形如 `postgres://user:pass@host:port/db` 的 URL。
pub async fn init_db(url: &str) -> Result<(), DbErr> {
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(MAX_CONNECTIONS)
        .min_connections(MIN_CONNECTIONS)
        .connect_timeout(CONNECT_TIMEOUT)
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .idle_timeout(IDLE_TIMEOUT)
        .max_lifetime(MAX_LIFETIME)
        // codegen 的 SQL 基本都是元数据查询，默认不打 SQL 日志，需要时用 sqlx_logging(true)
        .sqlx_logging(false);

    let conn = Database::connect(opt).await?;
    conn.ping().await?;
    DB.set(conn).map_err(|_| DbErr::Custom("codegen db already initialized".into()))?;
    log::info!("[codegen] sea-orm pool ready");
    Ok(())
}

/// 复用外部连接池（嵌进 backend 的场景）。
///
/// `DatabaseConnection` 内部是 `sqlx::Pool`，clone 成本很低。
pub fn init_external(conn: &DatabaseConnection) {
    let _ = DB.set(conn.clone());
}

/// 取连接。未初始化返回 `None`。
pub fn db() -> Option<&'static DatabaseConnection> {
    DB.get()
}

#[macro_export]
macro_rules! pool {
    () => {
        $crate::db::db().expect("[codegen] sea-orm db not initialized")
    };
}

pub async fn close_db() {
    if let Some(conn) = DB.get() {
        let _ = conn.clone().close().await;
    }
}
