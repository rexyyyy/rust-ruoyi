//! `job` —— 业务模块：动态定时任务（对标 RuoYi `ruoyi-quartz` / `sys_job`）。
//!
//! 基于自研的**方言无关轮询调度器**，不依赖 apalis 或任何数据库专属的队列/通知机制：
//! - `service::start_scheduler`：一个 tokio 后台任务每秒扫描 `status='0'` 的任务，
//!   按 `cron_expression` 计算应触发时刻，直接执行（无消息队列、无 LISTEN/NOTIFY）。
//! - 立即执行（`/job/execute`、`SysJobService::run`）也是直接调用 `service::execute_job`。
//! - 执行逻辑是日志桩 + 写 `sys_job_log`；`invoke_target` 仅被记录，尚未实现按 target
//!   反射调用具体业务方法（与原 apalis 实现行为一致，属后续扩展点）。
//!
//! 因为不依赖数据库专属机制，本模块在 PostgreSQL 与 MySQL 下行为完全一致，
//! 不再需要 feature 级 gating（`postgresql` / `mysql` 仅控制 sea-orm 的 SQL 驱动）。
//!
//! 启动流程见 `crate::main`：`ensure_tables` 建表，`start_scheduler` 拉起调度循环；
//! `init_engine` / `start_workers` 为历史兼容 no-op（轮询调度复用共享连接，无需独立池/worker）。
pub mod controller;
pub mod entity;
pub mod dto;
pub mod service;

/// 历史兼容：原 apalis 引擎需要独立 PG 连接池；轮询调度器直接复用共享连接（`state::db()`），
/// 无需建池，故此处为 no-op。保留签名以满足 `main` 启动流程。
pub async fn init_engine(_database_url: &str) -> Result<(), String> {
    log::info!("[job] 使用方言无关轮询调度器（无 apalis 依赖）");
    Ok(())
}

/// 历史兼容：worker 由 `start_scheduler` 内部的 tokio 任务承担，无需单独拉起，故 no-op。
pub async fn start_workers() -> Result<(), String> {
    Ok(())
}
