//! 全局 sea-orm 连接。
//!
//! 现已并入 [`crate::state::AppState`]：本模块只做「连接建立」与「遗留访问入口」
//! 两层适配。`init_db()` 建好连接后，连同 redis 客户端与配置一起写入 AppState；
//! `db()` / `close_db()` 仅转发到 [`crate::state`]，保证 `pool!()` 宏与既有
//! service 代码零改动继续可用。

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbBackend, DbErr, Statement, Value};
use once_cell::sync::OnceCell;
use std::time::Duration;

/// 连接池参数（集中在这里调整，不要散落到各 service）
/// 最大连接按 CPU 核数自适应：worker 越多池越大，避免高并发下 acquire 排队。
const MAX_CONNECTIONS_MIN: u32 = 20;
const MAX_CONNECTIONS_MAX: u32 = 100;
const MIN_CONNECTIONS: u32 = 5;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(10);
const IDLE_TIMEOUT: Duration = Duration::from_secs(600);
const MAX_LIFETIME: Duration = Duration::from_secs(1800);

/// 由连接串 scheme 推断 sea-orm 后端类型（与 sea-orm 自身按 scheme 选驱动的逻辑一致）：
/// `mysql://` → MySql；`postgres://` / `postgresql://` → Postgres；其余默认 Postgres。
///
/// 不依赖 `DatabaseConnection::get_database_backend`（该方法是 `ConnectionTrait` trait 方法，
/// 未把 trait 引入作用域时编译器报 E0599），直接从连接串判定更稳健、与版本无关。
fn backend_from_url(url: &str) -> DbBackend {
    match url.split("://").next() {
        Some("mysql") => DbBackend::MySql,
        Some("sqlite") => DbBackend::Sqlite,
        _ => DbBackend::Postgres,
    }
}

/// 建立数据库连接，并写入全局 [`crate::state::AppState`]（同时纳入 redis 客户端与配置）。
///
/// - 显式配置池参数，不再吃 sea-orm 默认值；
/// - `sqlx_logging(false)`：默认会把每条 SQL 语句打进日志，噪声极大，
///   排查慢查询时临时改成 `true` 即可；
/// - `ping()` 做连通性验证，连不上就在启动阶段失败（fail fast），
///   而不是拖到业务代码里 `pool!()` 的 `expect` 才 panic；
/// - redis 客户端从 `CONTEXT.config.web_redis_url` 另开一个（与 `cache_service`
///   共享同一 Redis 服务，客户端廉价可 Clone）；
/// - 返回 `Result`，由调用方决定是否终止启动。
pub async fn init_db() -> Result<(), DbErr> {
    let cfg = &crate::modules::system::service::CONTEXT.config;
    let url = cfg.database_url.clone();

    let workers = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4);
    let max_connections = (workers * 4).clamp(MAX_CONNECTIONS_MIN, MAX_CONNECTIONS_MAX);

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(max_connections)
        .min_connections(MIN_CONNECTIONS)
        .connect_timeout(CONNECT_TIMEOUT)
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .idle_timeout(IDLE_TIMEOUT)
        .max_lifetime(MAX_LIFETIME)
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let conn = Database::connect(opt).await?;
    // 记录实际后端（由连接串 scheme 决定），供 raw_stmt 等方言辅助使用。
    let _ = ACTIVE_BACKEND.set(backend_from_url(&cfg.database_url));
    conn.ping().await?;

    // redis 客户端：复用配置里的 web_redis_url（与 CONTEXT.cache_service 共享同一 Redis）
    let redis = redis::Client::open(cfg.web_redis_url.clone())
        .map_err(|e| DbErr::Custom(format!("redis client init failed: {e}")))?;

    crate::state::init(conn, redis, cfg.clone()).map_err(DbErr::Custom)?;

    log::info!(
        "[ruoyi_rust] sea-orm pool ready (max={}, min={}, workers={})",
        max_connections,
        MIN_CONNECTIONS,
        workers
    );
    Ok(())
}

/// 取全局连接。未初始化时返回 `None`，调用方应降级处理。
///
/// 实际来源是 [`crate::state::AppState`]，这里仅做转发以保持 `pool!()` 兼容。
pub fn db() -> Option<&'static DatabaseConnection> {
    crate::state::db()
}

/// 优雅关闭：停止后台回收任务并等待连接归还。进程退出前调用。
pub async fn close_db() {
    crate::state::close().await;
}

/// 全局激活的后端类型。在 [`init_db`] 连接成功后写入，
/// 供 [`raw_stmt`] 等需要「按实际后端生成方言」的原生 SQL 辅助使用。
static ACTIVE_BACKEND: OnceCell<DbBackend> = OnceCell::new();

/// 取当前激活的后端；未初始化时回退 PostgreSQL（与历史默认行为一致）。
pub fn active_backend() -> DbBackend {
    ACTIVE_BACKEND.get().copied().unwrap_or(DbBackend::Postgres)
}

/// 按实际后端构造 SeaORM [`Statement`]，并自动把 PostgreSQL 的 `$N` 占位符
/// 改写为 MySQL 的 `?`（值按位置对应，故改写安全）。
///
/// 所有手写原生 SQL 都应经此函数，而非直接 `Statement::from_sql_and_values(DbBackend::Postgres, ...)`，
/// 这样切换 MySQL 时无需逐个改 SQL。SeaORM query builder（paginate/count 等）本身方言无关，不受影响。
pub fn raw_stmt<S: AsRef<str>>(sql: S, vals: Vec<Value>) -> Statement {
    let backend = active_backend();
    let sql = if backend == DbBackend::MySql {
        pg_to_mysql_sql(sql.as_ref())
    } else {
        sql.as_ref().to_string()
    };
    Statement::from_sql_and_values(backend, &sql, vals)
}

/// 无参数版 [`raw_stmt`]，用于没有绑定值的原生 SQL（如 `delete from ...`）。
pub fn raw_stmt0<S: AsRef<str>>(sql: S) -> Statement {
    raw_stmt(sql, vec![])
}

/// 把 PostgreSQL 原生 SQL 中的 `$1`/`$2`/... 占位符改写为 MySQL 的 `?`。
/// 仅替换 `$` 后紧跟数字的情形，避免误伤普通文本。
fn pg_to_mysql_sql(sql: &str) -> String {
    // 先把 PG 专属类型转换映射到 MySQL 等价类型（仅此一处 CAST 用到）。
    let sql = sql
        .replace(" AS TEXT", " AS CHAR")
        .replace(" AS BIGINT", " AS UNSIGNED");
    let mut out = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            let mut digit_run = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    digit_run.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            if !digit_run.is_empty() {
                out.push('?');
            } else {
                out.push('$');
            }
        } else {
            out.push(c);
        }
    }
    out
}
