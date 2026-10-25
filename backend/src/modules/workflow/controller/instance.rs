//! 流程实例接口 `/workflow/instance/**`

use actix_web::{delete, get, put, web, Responder};

use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::workflow::domain::bo::InstancePageBo;
use crate::modules::workflow::service::{instance, leave};

#[get("/instance/list")]
pub async fn page(query: web::Query<InstancePageBo>) -> impl Responder {
    let data = instance::page(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 流程图（节点 / 连线带办理状态）
#[get("/instance/flowChart/{id}")]
pub async fn flow_chart(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => instance::chart(id).await,
        Err(_) => Err(crate::error::Error::from("流程实例 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

/// 流程变量
#[get("/instance/variable/{id}")]
pub async fn variable(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => instance::variables(id).await,
        Err(_) => Err(crate::error::Error::from("流程实例 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

/// 按业务主键查实例（业务列表页判断「是否已发起 / 当前节点」）
#[get("/instance/business/{business_id}")]
pub async fn by_business(path: web::Path<String>) -> impl Responder {
    let data = instance::find_by_business(&path.into_inner()).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/instance/active/{id}")]
pub async fn active(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => instance::set_activity(id, true).await,
        Err(_) => Err(crate::error::Error::from("流程实例 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[put("/instance/unActive/{id}")]
pub async fn un_active(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => instance::set_activity(id, false).await,
        Err(_) => Err(crate::error::Error::from("流程实例 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[get("/instance/{id}")]
pub async fn detail(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => instance::detail(id).await,
        Err(_) => Err(crate::error::Error::from("流程实例 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[delete("/instance/{ids}")]
pub async fn remove(path: web::Path<String>) -> impl Responder {
    let mut ids = Vec::new();
    for part in path.into_inner().split(',') {
        if let Ok(id) = part.trim().parse::<i64>() {
            ids.push(id);
        }
    }
    let data = instance::remove(&ids).await;
    RespVO::from_result(&data).resp_json()
}

/// 请假单 + 流程实例的组合详情（流程详情页直接渲染业务表单）
#[get("/instance/leave/{business_id}")]
pub async fn leave_detail(path: web::Path<String>) -> impl Responder {
    let business_id = path.into_inner();
    let data = leave::find_by_business(&business_id).await;
    RespVO::from_result(&data).resp_json()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(flow_chart)
        .service(variable)
        .service(by_business)
        .service(active)
        .service(un_active)
        .service(leave_detail)
        // 动态路由收尾
        .service(detail)
        .service(remove);
}
