//! /tool/gen/* 路由（对齐 plus-ui-5.X 前端 api/tool/gen 契约）。
//!
//! actix 按注册顺序匹配：固定路径全部前置，动态 `/{id}` 收尾。

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::tool::service::GenService;

fn service() -> GenService {
    GenService {}
}

/// 生成配置分页
#[get("/list")]
pub async fn list(query: web::Query<crate::modules::tool::domain::GenQuery>) -> impl Responder {
    let data = service().page_list(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 可导入的物理表分页
#[get("/db/list")]
pub async fn db_list(query: web::Query<crate::modules::tool::domain::GenQuery>) -> impl Responder {
    let data = service().db_list(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 数据源名称
#[get("/getDataNames")]
pub async fn get_data_names() -> impl Responder {
    let names = service().data_names().await;
    RespVO::from(&names).resp_json()
}

/// 导入表
#[post("/importTable")]
pub async fn import_table(query: web::Query<crate::modules::tool::domain::ImportTableQuery>) -> impl Responder {
    let data_name = query.data_name.clone().unwrap_or_else(|| "master".to_string());
    let affected = match service().import_tables(&query.tables, &data_name).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "导入失败！".to_string()).resp_json()
}

/// 预览生成代码
#[get("/preview/{table_id}")]
pub async fn preview(id: web::Path<i64>) -> impl Responder {
    let data = service().preview(id.into_inner()).await;
    match data {
        Ok(files) => {
            let map: std::collections::BTreeMap<String, String> = files.into_iter().collect();
            RespVO::from(&map).resp_json()
        }
        Err(e) => RespVO::<String>::from_error_info(500, &e.to_string()).resp_json(),
    }
}

/// 同步数据库结构
#[get("/synchDb/{table_id}")]
pub async fn synch_db(id: web::Path<i64>) -> impl Responder {
    match service().sync_db(id.into_inner()).await {
        Ok(n) => RespVO::<u64>::from_success_info(&format!("同步成功，变更 {} 列", n)).resp_json(),
        Err(e) => RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    }
}

/// 批量生成代码（zip 下载）
#[get("/batchGenCode")]
pub async fn batch_gen_code(query: web::Query<TableIdStr>) -> impl Responder {
    let ids: Vec<i64> = query
        .table_id_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    match service().batch_gen(&ids).await {
        Ok(bytes) => {
            let filename = format!("codegen-{}.zip", chrono::Local::now().format("%Y%m%d%H%M%S"));
            HttpResponse::Ok()
                .insert_header(("Content-Type", "application/zip"))
                .insert_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", filename),
                ))
                .body(bytes)
        }
        Err(e) => RespVO::<String>::from_error_info(500, &e.to_string()).resp_json(),
    }
}

/// 详情（info + rows + tables）
#[get("/{table_id}")]
pub async fn detail(id: web::Path<i64>) -> impl Responder {
    let vo = service().get_info(id.into_inner()).await;
    RespVO::from_result(&vo).resp_json()
}

/// 删除配置（支持逗号分隔多个 id）
#[delete("/{table_ids}")]
pub async fn remove(ids: web::Path<String>) -> impl Responder {
    let id_list: Vec<i64> = ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    let mut affected = 0u64;
    for id in &id_list {
        match service().remove(*id).await {
            Ok(v) => affected += v,
            Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
        }
    }
    RespVO::<u64>::judge(affected, String::new(), "删除失败！".to_string()).resp_json()
}

/// 修改生成配置（PUT /tool/gen，scope 根路由）
#[put("")]
pub async fn update(arg: web::Json<crate::modules::tool::domain::GenTableForm>) -> impl Responder {
    let table_id = arg.table_id;
    let affected = match service().update(table_id, arg.0).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "修改失败！".to_string()).resp_json()
}

#[derive(Deserialize, Debug)]
pub struct TableIdStr {
    #[serde(rename = "tableIdStr", default)]
    pub table_id_str: String,
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(db_list)
        .service(get_data_names)
        .service(import_table)
        .service(preview)
        .service(synch_db)
        .service(batch_gen_code)
        .service(detail)
        .service(remove)
        .service(update);
}
