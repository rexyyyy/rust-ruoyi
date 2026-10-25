//! 流程定义接口 `/workflow/definition/**`
//!
//! 设计器存盘的「定义 + 节点 + 连线」整份 JSON 由这里进出；
//! 引擎侧的 `/warm-flow/**` 接口是它的低层版本（warm-flow 原生设计器用）。

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::workflow::domain::bo::{DefinitionImportBo, DefinitionPageBo};
use crate::modules::workflow::service::definition;

/// 设计器数据入参：新建时不传 id
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DesignQuery {
    pub id: Option<String>,
    pub definition_id: Option<String>,
}

/// 保存时只覆盖节点与连线（不改流程基本信息）
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveQuery {
    pub only_node_skip: Option<bool>,
}

#[get("/definition/list")]
pub async fn page(query: web::Query<DefinitionPageBo>) -> impl Responder {
    let data = definition::page(&query).await;
    PageVO::from_result(&data).resp_json()
}

#[get("/definition/publishedList")]
pub async fn published_list() -> impl Responder {
    let data = definition::published_list().await;
    RespVO::from_result(&data).resp_json()
}

/// 设计器数据：不传 id 返回新建壳子（含分类树、表单路径、流程图形配置）
#[get("/definition/design")]
pub async fn design(query: web::Query<DesignQuery>) -> impl Responder {
    let raw = query.id.as_deref().or(query.definition_id.as_deref());
    let id = match raw.map(str::trim).filter(|s| !s.is_empty()) {
        None => None,
        Some(s) => match s.parse::<i64>() {
            Ok(id) => Some(id),
            Err(_) => {
                return RespVO::<serde_json::Value>::from_error_info(500, "流程定义 id 不合法").resp_json()
            }
        },
    };
    let data = definition::design_value(id).await;
    RespVO::from_result(&data).resp_json()
}

/// 保存设计器 JSON
#[post("/definition/design")]
pub async fn save(arg: web::Json<serde_json::Value>, query: web::Query<SaveQuery>) -> impl Responder {
    let data = definition::save_value(&arg.0, query.only_node_skip.unwrap_or(false)).await;
    RespVO::from_result(&data).resp_json()
}

/// 导入 JSON 字符串
#[post("/definition/import")]
pub async fn import(arg: web::Json<DefinitionImportBo>) -> impl Responder {
    let json = arg.json.as_deref().unwrap_or_default();
    if json.trim().is_empty() {
        return RespVO::<String>::from_error_info(500, "导入内容不能为空").resp_json();
    }
    let data = definition::import(json).await;
    RespVO::from_result(&data).resp_json()
}

/// 导出 JSON（浏览器下载）
#[get("/definition/export/{id}")]
pub async fn export(path: web::Path<String>) -> HttpResponse {
    let id = match path.into_inner().parse::<i64>() {
        Ok(id) => id,
        Err(_) => return RespVO::<String>::from_error_info(500, "流程定义 id 不合法").resp_json(),
    };
    match definition::export(id).await {
        Ok((file_name, json)) => HttpResponse::Ok()
            .insert_header(("Content-Type", "application/json;charset=UTF-8"))
            .insert_header((
                "Content-Disposition",
                format!("attachment; filename=\"{file_name}\""),
            ))
            .body(json),
        Err(e) => RespVO::<String>::from_error_info(500, &e.to_string()).resp_json(),
    }
}

#[put("/definition/publish/{id}")]
pub async fn publish(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => definition::publish(id).await,
        Err(_) => Err(crate::error::Error::from("流程定义 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[put("/definition/unPublish/{id}")]
pub async fn unpublish(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => definition::unpublish(id).await,
        Err(_) => Err(crate::error::Error::from("流程定义 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[post("/definition/copy/{id}")]
pub async fn copy(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => definition::copy(id).await,
        Err(_) => Err(crate::error::Error::from("流程定义 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[put("/definition/active/{id}")]
pub async fn active(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => definition::set_activity(id, true).await,
        Err(_) => Err(crate::error::Error::from("流程定义 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[put("/definition/unActive/{id}")]
pub async fn un_active(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => definition::set_activity(id, false).await,
        Err(_) => Err(crate::error::Error::from("流程定义 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[get("/definition/{id}")]
pub async fn detail(path: web::Path<String>) -> impl Responder {
    let data = match path.into_inner().parse::<i64>() {
        Ok(id) => definition::detail(id).await,
        Err(_) => Err(crate::error::Error::from("流程定义 id 不合法")),
    };
    RespVO::from_result(&data).resp_json()
}

#[delete("/definition/{ids}")]
pub async fn remove(path: web::Path<String>) -> impl Responder {
    let raw = path.into_inner();
    let mut ids = Vec::new();
    for part in raw.split(',') {
        if let Ok(id) = part.trim().parse::<i64>() {
            ids.push(id);
        }
    }
    let data = definition::remove(&ids).await;
    RespVO::from_result(&data).resp_json()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(published_list)
        .service(design)
        .service(save)
        .service(import)
        .service(export)
        .service(publish)
        .service(unpublish)
        .service(copy)
        .service(active)
        .service(un_active)
        // 动态路由收尾
        .service(detail)
        .service(remove);
}
