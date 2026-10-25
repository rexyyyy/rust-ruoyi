//! sea-orm 连接池。
//!
//! 与 `backend::db` 的做法保持一致（`ConnectOptions` + `OnceCell` + `ping()` fail-fast），
//! 这样 codegen 单独跑（CLI）和被 backend 内嵌调用时行为一致。
//!
//! 注意：被 backend 内嵌使用时，**不需要**调用 [`init_db`]，由 [`init_external`]
//! 直接复用宿主进程已有的连接池，避免同一个进程建两个池。

use std::time::Duration;

use once_cell::sync::OnceCell;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement, Value,
};

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

const MAX_CONNECTIONS: u32 = 8;
const MIN_CONNECTIONS: u32 = 1;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(10);
const IDLE_TIMEOUT: Duration = Duration::from_secs(600);
const MAX_LIFETIME: Duration = Duration::from_secs(1800);

/// 自建连接池（CLI 场景）。传入 `mysql://user:pass@host:port/db` 或
/// `postgres://user:pass@host:port/db`（scheme 决定方言，需与编译期驱动 feature 匹配）。
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

/// 当前连接的真实后端（由连接串 scheme 决定，与 `backend::db::active_backend` 同语义）。
///
/// 元数据 SQL 的 PG 系统表 / MySQL information_schema 分支就是靠它选择，
/// 不要再在 SQL 构造处写死 `DbBackend::Postgres`。
pub fn backend() -> DbBackend {
    crate::pool!().get_database_backend()
}

/// 当前连接是否为 MySQL 方言。
pub fn is_mysql() -> bool {
    backend() == DbBackend::MySql
}

/// 按**当前连接的真实后端**构造语句，调用方不需要（也不应该）指定方言。
///
/// 约定：元数据 SQL 统一用 PostgreSQL 风格书写（`$1` / `$2` 占位符），
/// MySQL 下在这里自动改写成 `?`，值顺序不变。方言专有的函数与类型转换
/// （如 MySQL 的 `DATABASE()`、`CAST(... AS SIGNED)`）仍由各分支自己的 SQL 负责。
pub fn stmt(sql: &str, vals: Vec<Value>) -> Statement {
    let backend = backend();
    let sql = if backend == DbBackend::MySql {
        pg_placeholders_to_mysql(sql)
    } else {
        sql.to_string()
    };
    Statement::from_sql_and_values(backend, &sql, vals)
}

/// 把 `$1`/`$2`/... 占位符改写为 MySQL 的 `?`。
/// 仅替换 `$` 后紧跟数字的情形，避免误伤普通文本。
fn pg_placeholders_to_mysql(sql: &str) -> String {
    let mut out = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            let mut digits = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    digits.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            if digits.is_empty() {
                out.push('$');
            } else {
                out.push('?');
            }
        } else {
            out.push(c);
        }
    }
    out
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
