//! 登录日志 / 操作日志的批量异步落库（审计日志写入通道）。
//!
//! 原实现是「每条日志 `tokio::spawn` 一次 `INSERT`」：
//! - 登录、退出、增删改等操作各触发一次，高并发下每秒产生几十个独立事务，
//!   每个事务都要占用连接池连接 + 走一次网络往返 + WAL fsync；
//! - `spawn` 没有上限，DB 变慢时任务越积越多，连接池被日志写打满，
//!   反而拖垮业务查询（日志写入本应是最不重要的一环）。
//!
//! 这里改成「有界队列 + 后台批量写」：
//! - 请求侧只做一次 `try_send`（无锁竞争、不 await、不占连接）；
//! - 后台任务按 [`BATCH_MAX`] 或 [`FLUSH_INTERVAL`] 聚合，一条 `INSERT ... VALUES (...),(...)`
//!   写入整批，把 N 次往返压成 1 次；
//! - 队列满时**丢弃并告警**（有界背压），保证 DB 故障时不会拖垮主流程；
//!   审计日志的可用性优先级低于业务可用性。

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use once_cell::sync::OnceCell;
use sea_orm::entity::*;
use sea_orm::IntoActiveModel;
use tokio::sync::mpsc;

use crate::modules::system::domain::{
    SysLogininfor, SysLogininforActiveModel, SysLogininforEntity, SysOperLog,
    SysOperLogActiveModel, SysOperLogEntity,
};

/// 队列容量：正常流量下远达不到，仅在 DB 长时间不可用时起到限流作用。
const QUEUE_CAPACITY: usize = 8192;
/// 单批最大条数（PG 参数上限 65535，16 列 × 200 = 3200，安全）。
const BATCH_MAX: usize = 200;
/// 攒批窗口：日志延迟 500ms 落库，对审计场景无感。
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);

struct Sinks {
    login: mpsc::Sender<SysLogininfor>,
    oper: mpsc::Sender<SysOperLog>,
}

static SINKS: OnceCell<Sinks> = OnceCell::new();
/// 运行期丢弃计数（队列满 / 未初始化），用于告警限频。
static DROPPED: AtomicU64 = AtomicU64::new(0);

/// 启动两个写入协程。应在 tokio runtime 内、DB 初始化之后调用一次（重复调用无副作用）。
pub fn init() {
    if SINKS.get().is_some() {
        return;
    }
    let (login_tx, login_rx) = mpsc::channel::<SysLogininfor>(QUEUE_CAPACITY);
    let (oper_tx, oper_rx) = mpsc::channel::<SysOperLog>(QUEUE_CAPACITY);
    if SINKS.set(Sinks { login: login_tx, oper: oper_tx }).is_err() {
        return;
    }
    tokio::spawn(run_login_writer(login_rx));
    tokio::spawn(run_oper_writer(oper_rx));
    log::info!("[log-sink] 审计日志批量写入通道已启动（batch={BATCH_MAX}, flush={FLUSH_INTERVAL:?}）");
}

/// 投递一条登录日志（非阻塞，队列满则丢弃并告警）。
pub fn push_logininfor(info: SysLogininfor) {
    let Some(sinks) = SINKS.get() else {
        // 未初始化（测试 / 启动早期）：退化为一次性异步插入，保证不丢日志
        tokio::spawn(async move {
            if let Some(db) = crate::state::db() {
                let _ = insert_one::<SysLogininforEntity>(db, info.into_active_model()).await;
            }
        });
        return;
    };
    if let Err(mpsc::error::TrySendError::Full(info)) = sinks.login.try_send(info) {
        drop_one("sys_logininfor", &info.user_name);
    }
}

/// 投递一条操作日志（非阻塞）。
pub fn push_operlog(oper: SysOperLog) {
    let Some(sinks) = SINKS.get() else {
        tokio::spawn(async move {
            if let Some(db) = crate::state::db() {
                let _ = insert_one::<SysOperLogEntity>(db, oper.into_active_model()).await;
            }
        });
        return;
    };
    if let Err(mpsc::error::TrySendError::Full(oper)) = sinks.oper.try_send(oper) {
        drop_one("sys_oper_log", &oper.title);
    }
}

fn drop_one(table: &str, what: &Option<String>) {
    let n = DROPPED.fetch_add(1, Ordering::Relaxed) + 1;
    // 每 100 条告警一次，避免刷屏
    if n % 100 == 1 {
        log::warn!("[log-sink] 队列已满，丢弃审计日志 table={table} sample={what:?} total_dropped={n}");
    }
}

async fn insert_one<E>(db: &'static sea_orm::DatabaseConnection, am: E::ActiveModel) -> Result<(), sea_orm::DbErr>
where
    E: sea_orm::EntityTrait,
{
    E::insert(am).exec(db).await.map(|_| ())
}

// ---------------------------------------------------------------------------
// 后台批量写入
// ---------------------------------------------------------------------------

async fn run_login_writer(mut rx: mpsc::Receiver<SysLogininfor>) {
    let mut buf: Vec<SysLogininfor> = Vec::with_capacity(BATCH_MAX);
    loop {
        let Some(first) = rx.recv().await else { break };
        buf.push(first);
        drain_window(&mut rx, &mut buf).await;
        flush_login(&mut buf).await;
    }
}

async fn run_oper_writer(mut rx: mpsc::Receiver<SysOperLog>) {
    let mut buf: Vec<SysOperLog> = Vec::with_capacity(BATCH_MAX);
    loop {
        let Some(first) = rx.recv().await else { break };
        buf.push(first);
        drain_window(&mut rx, &mut buf).await;
        flush_oper(&mut buf).await;
    }
}

/// 在 [`FLUSH_INTERVAL`] 窗口内尽量多收集，攒满 [`BATCH_MAX`] 立即返回。
async fn drain_window<T>(rx: &mut mpsc::Receiver<T>, buf: &mut Vec<T>) {
    let deadline = tokio::time::sleep(FLUSH_INTERVAL);
    tokio::pin!(deadline);
    while buf.len() < BATCH_MAX {
        tokio::select! {
            maybe = rx.recv() => match maybe {
                Some(v) => buf.push(v),
                None => return, // 通道关闭
            },
            _ = &mut deadline => return,
        }
    }
}

async fn flush_login(buf: &mut Vec<SysLogininfor>) {
    if buf.is_empty() {
        return;
    }
    let n = buf.len();
    let Some(db) = crate::state::db() else {
        buf.clear();
        return;
    };
    let ams: Vec<SysLogininforActiveModel> = buf.drain(..).map(|m| m.into_active_model()).collect();
    if let Err(e) = SysLogininforEntity::insert_many(ams).exec(db).await {
        // 不重试：DB 故障时重试只会放大压力，且日志可接受丢失
        log::warn!("[log-sink] 批量写入 sys_logininfor 失败（{n} 条）: {e}");
    }
}

async fn flush_oper(buf: &mut Vec<SysOperLog>) {
    if buf.is_empty() {
        return;
    }
    let n = buf.len();
    let Some(db) = crate::state::db() else {
        buf.clear();
        return;
    };
    let ams: Vec<SysOperLogActiveModel> = buf.drain(..).map(|m| m.into_active_model()).collect();
    if let Err(e) = SysOperLogEntity::insert_many(ams).exec(db).await {
        log::warn!("[log-sink] 批量写入 sys_oper_log 失败（{n} 条）: {e}");
    }
}
