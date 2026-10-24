//! 动态定时任务服务（对标 RuoYi sys_job / sys_job_log）。
//!
//! - 任务定义持久化在 `sys_job`（sea-orm 实体见 `entity`）。
//! - 动态调度：`start_scheduler` 每秒扫描 `status='0'` 的任务，按 `cron_expression`
//!   计算应触发时刻，直接 `execute_job`（方言无关轮询，无 apalis / 消息队列）。
//! - 暂停/恢复 = 改 `status`；增删改 = 行操作；立即执行 = 直接 `execute_job`。
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use chrono::{Utc, DateTime, NaiveDateTime};
use cron::Schedule;
use std::str::FromStr;
use sea_orm::*;

use crate::modules::job::entity::{
    SysJob, SysJobActiveModel, SysJobEntity, SysJobColumn,
    SysJobLog, SysJobLogEntity, SysJobLogColumn, SysJobLogActiveModel,
};
use crate::modules::job::dto::{JobPageDTO, JobAddDTO, JobUpdateDTO, JobLogPageDTO};
use crate::modules::system::dao::*;
use crate::modules::system::domain::{Page, PageRequest};
use crate::state;
use crate::error::{Error, Result};

/// 每个任务上次被判定应触发的时间（用于避免重复触发 / 计算下一触发点）。
static LAST_FIRE: Lazy<Mutex<HashMap<i64, DateTime<Utc>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
/// 并发受限（concurrent='0'）任务当前是否仍在执行中。
static INFLIGHT: Lazy<Mutex<HashMap<i64, ()>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// cron 表达式解析结果缓存（表达式原文 -> Schedule）。
///
/// 调度器每秒 tick 一次，若每个任务都重新 `Schedule::from_str`，就是
/// 「每秒 × 任务数」次表达式解析（内部要做字段切分与取值域构造），纯属浪费。
/// 表达式几乎不变，缓存后仅在首次出现时解析一次；键的数量等于不同表达式的个数，有界。
static CRON_CACHE: Lazy<Mutex<HashMap<String, Schedule>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// 取（并缓存）cron 表达式的解析结果；表达式非法时返回 None（不缓存失败结果）。
fn cached_schedule(expr: &str) -> Option<Schedule> {
    let mut guard = match CRON_CACHE.lock() {
        Ok(g) => g,
        Err(_) => {
            // 锁中毒时退化为「每次现解析」，保证调度不中断
            return Schedule::from_str(expr).ok();
        }
    };
    if let Some(s) = guard.get(expr) {
        return Some(s.clone());
    }
    let s = Schedule::from_str(expr).ok()?;
    guard.insert(expr.to_string(), s.clone());
    Some(s)
}

#[inline]
fn db() -> Result<&'static DatabaseConnection> {
    state::db().ok_or_else(|| Error::from("db not initialized"))
}

/// 幂等建表：sys_job（任务定义）+ sys_job_log（执行日志）。
/// 按实际后端选择 DDL：PostgreSQL 建在 `public` schema 且用 BIGSERIAL；
/// MySQL 去 schema 前缀、改用 BIGINT AUTO_INCREMENT PRIMARY KEY + DATETIME。
pub async fn ensure_tables(conn: &DatabaseConnection) -> Result<()> {
    let pg = [
        "CREATE TABLE IF NOT EXISTS public.sys_job (\
            job_id BIGSERIAL PRIMARY KEY, \
            job_name VARCHAR(64) NOT NULL DEFAULT '', \
            job_group VARCHAR(64) NOT NULL DEFAULT 'DEFAULT', \
            invoke_target VARCHAR(500) NOT NULL DEFAULT '', \
            cron_expression VARCHAR(255) NOT NULL DEFAULT '', \
            misfire_policy VARCHAR(20) NOT NULL DEFAULT '3', \
            concurrent VARCHAR(1) NOT NULL DEFAULT '1', \
            status VARCHAR(1) NOT NULL DEFAULT '0', \
            create_by VARCHAR(64), \
            create_time TIMESTAMP, \
            update_by VARCHAR(64), \
            update_time TIMESTAMP, \
            remark VARCHAR(500))",
        "CREATE TABLE IF NOT EXISTS public.sys_job_log (\
            job_log_id BIGSERIAL PRIMARY KEY, \
            job_name VARCHAR(64), \
            job_group VARCHAR(64), \
            invoke_target VARCHAR(500), \
            job_message VARCHAR(500), \
            status VARCHAR(1) NOT NULL DEFAULT '0', \
            exception_info TEXT, \
            create_time TIMESTAMP)",
    ];
    let mysql = [
        "CREATE TABLE IF NOT EXISTS sys_job (\
            job_id BIGINT AUTO_INCREMENT PRIMARY KEY, \
            job_name VARCHAR(64) NOT NULL DEFAULT '', \
            job_group VARCHAR(64) NOT NULL DEFAULT 'DEFAULT', \
            invoke_target VARCHAR(500) NOT NULL DEFAULT '', \
            cron_expression VARCHAR(255) NOT NULL DEFAULT '', \
            misfire_policy VARCHAR(20) NOT NULL DEFAULT '3', \
            concurrent VARCHAR(1) NOT NULL DEFAULT '1', \
            status VARCHAR(1) NOT NULL DEFAULT '0', \
            create_by VARCHAR(64), \
            create_time DATETIME, \
            update_by VARCHAR(64), \
            update_time DATETIME, \
            remark VARCHAR(500))",
        "CREATE TABLE IF NOT EXISTS sys_job_log (\
            job_log_id BIGINT AUTO_INCREMENT PRIMARY KEY, \
            job_name VARCHAR(64), \
            job_group VARCHAR(64), \
            invoke_target VARCHAR(500), \
            job_message VARCHAR(500), \
            status VARCHAR(1) NOT NULL DEFAULT '0', \
            exception_info TEXT, \
            create_time DATETIME)",
    ];
    let sqls: &[&str] = if matches!(crate::db::active_backend(), DbBackend::MySql) {
        &mysql
    } else {
        &pg
    };
    for sql in sqls {
        conn.execute(crate::db::raw_stmt(*sql, vec![]))
            .await
            .map_err(Error::from)?;
    }
    Ok(())
}

pub struct SysJobService {}

impl SysJobService {
    pub async fn page(&self, dto: &JobPageDTO) -> Result<Page<SysJob>> {
        let conn = db()?;
        let pr = PageRequest::new(dto.page_no.unwrap_or(1), dto.page_size.unwrap_or(10));
        let mut select = SysJobEntity::find();
        if let Some(n) = &dto.job_name {
            if !n.is_empty() {
                select = select.filter(SysJobColumn::JobName.contains(n.as_str()));
            }
        }
        if let Some(g) = &dto.job_group {
            if !g.is_empty() {
                select = select.filter(SysJobColumn::JobGroup.eq(g.clone()));
            }
        }
        if let Some(s) = &dto.status {
            if !s.is_empty() {
                select = select.filter(SysJobColumn::Status.eq(s.clone()));
            }
        }
        if let Some(t) = &dto.invoke_target {
            if !t.is_empty() {
                select = select.filter(SysJobColumn::InvokeTarget.contains(t.as_str()));
            }
        }
        select = select.order_by_desc(SysJobColumn::JobId);
        paginate(conn, select, &pr).await
    }

    pub async fn detail(&self, job_id: i64) -> Result<SysJob> {
        let conn = db()?;
        SysJobEntity::find()
            .filter(SysJobColumn::JobId.eq(job_id))
            .one(conn)
            .await?
            .ok_or_else(|| Error::from(format!("任务不存在: {}", job_id)))
    }

    pub async fn add(&self, dto: JobAddDTO) -> Result<u64> {
        let conn = db()?;
        let now = Utc::now().naive_utc();
        let am = sys_job_active_model(dto, now);
        SysJobEntity::insert(am).exec(conn).await.map(|_| 1).map_err(Error::from)
    }

    pub async fn update(&self, dto: JobUpdateDTO) -> Result<u64> {
        let conn = db()?;
        let mut existing = SysJobEntity::find()
            .filter(SysJobColumn::JobId.eq(dto.job_id))
            .one(conn)
            .await?
            .ok_or_else(|| Error::from(format!("任务不存在: {}", dto.job_id)))?;
        if let Some(v) = dto.job_name {
            existing.job_name = Some(v);
        }
        if let Some(v) = dto.job_group {
            existing.job_group = Some(v);
        }
        if let Some(v) = dto.invoke_target {
            existing.invoke_target = Some(v);
        }
        if let Some(v) = dto.cron_expression {
            existing.cron_expression = Some(v);
        }
        if let Some(v) = dto.misfire_policy {
            existing.misfire_policy = Some(v);
        }
        if let Some(v) = dto.concurrent {
            existing.concurrent = Some(v);
        }
        if let Some(v) = dto.status {
            existing.status = Some(v);
        }
        if let Some(v) = dto.remark {
            existing.remark = Some(v);
        }
        existing.update_by = dto.update_by;
        existing.update_time = Some(Utc::now().naive_utc());
        update_model(conn, existing).await
    }

    pub async fn remove(&self, job_id: i64) -> Result<u64> {
        let conn = db()?;
        let r = SysJobEntity::delete_by_id(job_id).exec(conn).await.map_err(Error::from)?;
        Ok(r.rows_affected)
    }

    /// 暂停 / 恢复：status '0'=运行, '1'=暂停。
    pub async fn change_status(&self, job_id: i64, status: String) -> Result<u64> {
        let conn = db()?;
        if let Some(mut ej) = SysJobEntity::find()
            .filter(SysJobColumn::JobId.eq(job_id))
            .one(conn)
            .await?
        {
            ej.status = Some(status);
            ej.update_time = Some(Utc::now().naive_utc());
            return update_model(conn, ej).await;
        }
        Ok(0)
    }

    /// 立即执行一次：直接调用 `execute_job`（方言无关轮询，不再入 apalis 队列）。
    pub async fn run(&self, job_id: i64) -> Result<u64> {
        let conn = db()?;
        let m = SysJobEntity::find()
            .filter(SysJobColumn::JobId.eq(job_id))
            .one(conn)
            .await?
            .ok_or_else(|| Error::from(format!("任务不存在: {}", job_id)))?;
        execute_job(&m).await;
        Ok(1)
    }

    pub async fn log_page(&self, dto: &JobLogPageDTO) -> Result<Page<SysJobLog>> {
        let conn = db()?;
        let pr = PageRequest::new(dto.page_no.unwrap_or(1), dto.page_size.unwrap_or(10));
        let mut select = SysJobLogEntity::find();
        if let Some(n) = &dto.job_name {
            if !n.is_empty() {
                select = select.filter(SysJobLogColumn::JobName.contains(n.as_str()));
            }
        }
        if let Some(s) = &dto.status {
            if !s.is_empty() {
                select = select.filter(SysJobLogColumn::Status.eq(s.clone()));
            }
        }
        select = select.order_by_desc(SysJobLogColumn::JobLogId);
        paginate(conn, select, &pr).await
    }

    pub async fn remove_log(&self, log_id: i64) -> Result<u64> {
        let conn = db()?;
        let r = SysJobLogEntity::delete_by_id(log_id).exec(conn).await.map_err(Error::from)?;
        Ok(r.rows_affected)
    }

    pub async fn clean_logs(&self) -> Result<u64> {
        let conn = db()?;
        let r = SysJobLogEntity::delete_many().exec(conn).await.map_err(Error::from)?;
        Ok(r.rows_affected)
    }
}

/// 由新增 DTO 构造 ActiveModel（带默认值：status='0' 运行 / concurrent='1' 允许并发 / misfire='3'）。
fn sys_job_active_model(dto: JobAddDTO, now: NaiveDateTime) -> SysJobActiveModel {
    let empty_to = |v: Option<String>, d: &str| v.filter(|s| !s.is_empty()).unwrap_or_else(|| d.to_string());
    SysJobActiveModel {
        job_id: NotSet,
        job_name: Set(dto.job_name),
        job_group: Set(dto.job_group),
        invoke_target: Set(dto.invoke_target),
        cron_expression: Set(dto.cron_expression),
        misfire_policy: Set(Some(empty_to(dto.misfire_policy, "3"))),
        concurrent: Set(Some(empty_to(dto.concurrent, "1"))),
        status: Set(Some(empty_to(dto.status, "0"))),
        create_by: Set(dto.create_by),
        create_time: Set(Some(now)),
        update_by: NotSet,
        update_time: NotSet,
        remark: Set(dto.remark),
    }
}

/// 真正执行一个任务（对标原 apalis `worker::process_job` 的对等物）：
/// 按 `invoke_target` 记录分发日志，写一条 `sys_job_log`，并在并发受限时解除占用。
///
/// 注意：本仓库尚未实现「按 `invoke_target` 反射调用具体业务方法」——`process_job` 原本
/// 也只做日志 + 写日志表，故此处行为与原 apalis 实现一致（属后续扩展点）。
pub async fn execute_job(job: &SysJob) {
    let key = job.invoke_target.clone().unwrap_or_default();
    log::info!(
        "[job] execute job_id={} invoke_target={} (polling scheduler)",
        job.job_id, key
    );
    write_job_log(job.job_id, true).await;
    mark_done(job.job_id);
}

/// 按 `invoke_target` 立即执行（HTTP `/job/execute` 入口）：若能匹配到 `sys_job` 则走完整
/// 执行 + 日志流程；否则仅记录分发日志。返回值不影响接口契约。
pub async fn execute_by_key(key: &str, payload: Option<serde_json::Value>) -> Result<()> {
    log::info!("[job] execute by key={} payload={:?} (polling scheduler)", key, payload);
    let conn = db()?;
    if let Some(job) = SysJobEntity::find()
        .filter(SysJobColumn::InvokeTarget.eq(key))
        .one(conn)
        .await?
    {
        execute_job(&job).await;
    }
    Ok(())
}

/// 写入一条 sys_job_log（基于 sys_job 元数据）。
async fn write_job_log(job_id: i64, success: bool) {
    let Some(db) = state::db() else { return };
    let Ok(Some(m)) = SysJobEntity::find()
        .filter(SysJobColumn::JobId.eq(job_id))
        .one(db)
        .await
    else {
        return;
    };
    let status = if success { "0" } else { "1" }.to_string();
    let msg = if success { "执行成功" } else { "执行失败" }.to_string();
    let am = SysJobLogActiveModel {
        job_log_id: NotSet,
        job_name: Set(m.job_name.clone()),
        job_group: Set(m.job_group.clone()),
        invoke_target: Set(m.invoke_target.clone()),
        job_message: Set(Some(msg)),
        status: Set(Some(status)),
        exception_info: Set(None),
        create_time: Set(Some(Utc::now().naive_utc())),
    };
    if let Err(e) = SysJobLogEntity::insert(am).exec(db).await {
        log::warn!("[job] write_job_log failed: {e}");
    }
}

/// 执行完成后由 worker 调用，解除并发占用（concurrent='0' 时）。
pub fn mark_done(job_id: i64) {
    INFLIGHT.lock().unwrap().remove(&job_id);
}

/// 动态调度器：每秒扫描启用中的任务，按 cron 表达式触发执行。
///
/// 设计要点：
/// - 仅在 `status='0'` 的任务参与调度；`changeStatus` 改 status 即生效（下一次 tick 生效）。
/// - 用 `LAST_FIRE` 记录上次触发时刻，避免同一触发点在多个 tick 内重复触发。
/// - `concurrent='0'`（禁止并发）时，若上一次尚未执行完（INFLIGHT 中）则跳过本次。
/// - 启动时不回填历史到期任务（misfire=DO_NOTHING），`LAST_FIRE` 初值取当前时刻。
pub fn start_scheduler() {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            let now = Utc::now();
            let conn = match state::db() {
                Some(c) => c,
                None => continue,
            };
            let jobs = match SysJobEntity::find()
                .filter(SysJobColumn::Status.eq("0"))
                .all(conn)
                .await
            {
                Ok(j) => j,
                Err(e) => {
                    log::warn!("[job-scheduler] query failed: {e}");
                    continue;
                }
            };
            for job in jobs {
                let expr = match &job.cron_expression {
                    Some(e) if !e.is_empty() => e,
                    _ => continue,
                };
                // 走缓存：避免每秒为每个任务重复解析 cron 表达式
                let schedule = match cached_schedule(expr) {
                    Some(s) => s,
                    None => {
                        log::warn!("[job-scheduler] bad cron for job {}: {}", job.job_id, expr);
                        continue;
                    }
                };
                let concurrent_off = job.concurrent.as_deref() == Some("0");
                if concurrent_off && INFLIGHT.lock().unwrap().contains_key(&job.job_id) {
                    continue;
                }
                let should_fire = {
                    let mut lf = LAST_FIRE.lock().unwrap();
                    let last = lf.get(&job.job_id).copied().unwrap_or(now);
                    match schedule.after(&last).next() {
                        Some(next) if next <= now => {
                            lf.insert(job.job_id, next);
                            true
                        }
                        _ => false,
                    }
                };
                if should_fire {
                    if concurrent_off {
                        INFLIGHT.lock().unwrap().insert(job.job_id, ());
                    }
                    let job = job.clone();
                    tokio::spawn(async move {
                        execute_job(&job).await;
                    });
                }
            }
        }
    });
}
