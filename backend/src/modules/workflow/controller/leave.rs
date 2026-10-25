//! 请假单接口 `/workflow/leave/**`（工作流接入示例）
//!
//! `submit` 一步完成「保存单据 + 发起流程 + 回填申请编号」，
//! 对应 Java 版 `TestLeaveController.submit` + `FlwTaskController.startWorkFlow`。

use actix_web::{delete, get, post, put, web, Responder};

use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::workflow::domain::bo::{LeaveBo, LeavePageBo};
use crate::modules::workflow::service::leave;
use crate::web_data::CurrentUser;

#[get("/leave/list")]
pub async fn page(query: web::Query<LeavePageBo>) -> impl Responder {
    let data = leave::page(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 保存并提交审批
#[post("/leave/submit")]
pub async fn submit(arg: web::Json<LeaveBo>, user: CurrentUser) -> impl Responder {
    let data = leave::submit(&arg.0, user.id).await;
    RespVO::from_result(&data).resp_json()
}

#[post("/leave")]
pub async fn add(arg: web::Json<LeaveBo>, user: CurrentUser) -> impl Responder {
    // 主键以字符串下发：雪花 id 超出 JS 安全整数范围
    let data = leave::add(&arg.0, user.id).await.map(|id| id.to_string());
    RespVO::from_result(&data).resp_json()
}

#[put("/leave")]
pub async fn update(arg: web::Json<LeaveBo>, user: CurrentUser) -> impl Responder {
    let data = leave::update(&arg.0, user.id).await;
    RespVO::from_result(&data).resp_json()
}

#[get("/leave/{id}")]
pub async fn detail(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => leave::detail(id).await,
        Err(_) => Err(crate::error::Error::from("请假单 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[delete("/leave/{ids}")]
pub async fn remove(path: web::Path<String>) -> impl Responder {
    let mut ids = Vec::new();
    for part in path.into_inner().split(',') {
        if let Ok(id) = part.trim().parse::<i64>() {
            ids.push(id);
        }
    }
    let data = leave::remove(&ids).await;
    RespVO::from_result(&data).resp_json()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(submit)
        .service(add)
        .service(update)
        // 动态路由收尾
        .service(detail)
        .service(remove);
}
