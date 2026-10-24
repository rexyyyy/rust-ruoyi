//! job 模块 HTTP 接口（对标 RuoYi sys_job 动态任务管理：增删改 / 暂停恢复 / 立即执行）。
//!
//! 路由前缀说明：`http_server.rs` 已用 `web::scope("/job")` 包裹本 `init`，
//! 故所有路由相对路径拼接后为 `/dev-api/job/...`。
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::modules::job::dto::{JobPageDTO, JobAddDTO, JobUpdateDTO, JobStatusDTO, JobRunDTO, JobLogPageDTO};
use crate::modules::job::service::{SysJobService, execute_by_key};
use crate::modules::system::domain::vo::{PageVO, RespVO};

#[derive(Debug, Deserialize)]
pub struct JobExecuteReq {
    pub job_key: String,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

pub fn init(cfg: &mut web::ServiceConfig) {
    // 路由注册顺序很重要：actix 在本项目中按注册顺序匹配，单段通配 `/{job_id}`
    // 会「吞掉」排在它之后注册的单段静态路由（/run、/changeStatus 等 → 405）。
    // 因此所有具体静态路由必须排在 `/{job_id}` 之前；`/{job_id}` 放最后并补齐所需方法。
    // —— 原有：按 job_key 立即执行 + 引擎健康检查 ——
    cfg.service(web::resource("/execute").route(web::post().to(execute)));
    cfg.service(web::resource("/health").route(web::get().to(health)));
    // —— 对标 RuoYi sys_job 的动态任务管理（具体路由在前） ——
    cfg.service(web::resource("/list").route(web::get().to(page)));
    cfg.service(web::resource("/changeStatus").route(web::put().to(change_status)));
    cfg.service(web::resource("/run").route(web::put().to(run)));
    cfg.service(web::resource("").route(web::post().to(add)).route(web::put().to(update)));
    // —— 执行日志 ——
    cfg.service(web::resource("/log/list").route(web::get().to(log_page)));
    cfg.service(web::resource("/log/clean").route(web::delete().to(clean_logs)));
    cfg.service(web::resource("/log/{log_id}").route(web::delete().to(remove_log)));
    // —— 单段主键路由放最后（detail=GET，remove=DELETE） ——
    cfg.service(web::resource("/{job_id}").route(web::get().to(detail)).route(web::delete().to(remove)));
}

// ===== 按 job_key 立即执行 + 健康检查（自研轮询调度，方言无关） =====
async fn execute(req: web::Json<JobExecuteReq>) -> HttpResponse {
    match execute_by_key(&req.job_key, req.payload.clone()).await {
        Ok(_) => HttpResponse::Ok().json(json!({"code": 200, "msg": "executed", "data": null})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"code": 500, "msg": format!("execute failed: {}", e), "data": null})),
    }
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "code": 200,
        "msg": "polling scheduler alive",
        "data": null
    }))
}

// ===== 动态任务管理（RuoYi sys_job） =====
async fn page(query: web::Query<JobPageDTO>) -> impl Responder {
    let r = SysJobService {}.page(&query.0).await;
    PageVO::from_result(&r).resp_json()
}

async fn detail(job_id: web::Path<i64>) -> impl Responder {
    let r = SysJobService {}.detail(job_id.into_inner()).await;
    RespVO::from_result(&r).resp_json()
}

async fn add(arg: web::Json<JobAddDTO>) -> impl Responder {
    let r = SysJobService {}.add(arg.into_inner()).await;
    RespVO::from_result(&r).resp_json()
}

async fn update(arg: web::Json<JobUpdateDTO>) -> impl Responder {
    let r = SysJobService {}.update(arg.into_inner()).await;
    RespVO::from_result(&r).resp_json()
}

async fn remove(job_id: web::Path<i64>) -> impl Responder {
    let r = SysJobService {}.remove(job_id.into_inner()).await;
    RespVO::<u64>::judge(r.unwrap_or(0), "操作成功".into(), "删除失败".into()).resp_json()
}

async fn change_status(arg: web::Json<JobStatusDTO>) -> impl Responder {
    let r = SysJobService {}.change_status(arg.job_id, arg.status.clone()).await;
    RespVO::from_result(&r).resp_json()
}

async fn run(arg: web::Json<JobRunDTO>) -> impl Responder {
    let r = SysJobService {}.run(arg.job_id).await;
    RespVO::from_result(&r).resp_json()
}

async fn log_page(query: web::Query<JobLogPageDTO>) -> impl Responder {
    let r = SysJobService {}.log_page(&query.0).await;
    PageVO::from_result(&r).resp_json()
}

async fn clean_logs() -> impl Responder {
    let r = SysJobService {}.clean_logs().await;
    RespVO::from_result(&r).resp_json()
}

async fn remove_log(log_id: web::Path<i64>) -> impl Responder {
    let r = SysJobService {}.remove_log(log_id.into_inner()).await;
    RespVO::<u64>::judge(r.unwrap_or(0), "操作成功".into(), "删除失败".into()).resp_json()
}
