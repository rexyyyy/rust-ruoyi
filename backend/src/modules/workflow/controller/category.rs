//! 流程分类接口 `/workflow/category/**`

use actix_web::{delete, get, post, put, web, Responder};

use crate::modules::system::domain::vo::RespVO;
use crate::modules::workflow::domain::bo::{CategoryBo, CategoryQuery};
use crate::modules::workflow::service::category;
use crate::web_data::CurrentUser;

#[get("/category/list")]
pub async fn list(query: web::Query<CategoryQuery>) -> impl Responder {
    let data = category::list(query.category_name.as_deref()).await;
    RespVO::from_result(&data).resp_json()
}

#[get("/category/tree")]
pub async fn tree() -> impl Responder {
    let data = category::tree().await;
    RespVO::from_result(&data).resp_json()
}

#[get("/category/{category_id}")]
pub async fn detail(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => category::detail(id).await,
        Err(_) => Err(crate::error::Error::from("分类 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[post("/category")]
pub async fn add(arg: web::Json<CategoryBo>, user: CurrentUser) -> impl Responder {
    let data = category::add(&arg.0, user.id).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/category")]
pub async fn update(arg: web::Json<CategoryBo>, user: CurrentUser) -> impl Responder {
    let data = category::update(&arg.0, user.id).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/category/{category_id}")]
pub async fn remove(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => category::remove(id).await,
        Err(_) => Err(crate::error::Error::from("分类 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(tree)
        .service(add)
        .service(update)
        // 动态路由放最后，避免吞掉 /category/list、/category/tree
        .service(detail)
        .service(remove);
}
