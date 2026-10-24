use crate::modules::system::domain::bo::post::{PostAddDTO, PostUpdateDTO, PostPageDTO};
use crate::modules::system::domain::SysPost;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::CONTEXT;
use futures_util::future::BoxFuture;
use actix_web::{get, post, put, delete, web, Responder};
use crate::web_data::CurrentUser;
use crate::config::global_variables::STATUS_NORMAL;

#[get("/post/list")]
pub async fn page(query: web::Query<PostPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_post_service.page(&query).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/post/{post_id}")]
pub async fn detail(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let post_vo = CONTEXT.sys_post_service.detail(&dict_id).await;
    RespVO::from_result(&post_vo).resp_json()
}


#[post("/post")]
pub async fn add(arg: web::Json<PostAddDTO>, current_user: CurrentUser) -> impl Responder {
    let mut data = SysPost::from(arg.0);
    data.create_by = Some(current_user.id);
    if data.status.is_none() {
        data.status = Some(STATUS_NORMAL.to_string());
    }
    let data = CONTEXT.sys_post_service.add(&data).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/post")]
pub async fn update(arg: web::Json<PostUpdateDTO>, current_user: CurrentUser) -> impl Responder {
    let mut data = SysPost::from(arg.0);
    data.update_by = Some(current_user.id);
    let data = CONTEXT.sys_post_service.update(data).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/post/{post_id}")]
pub async fn remove(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let data = CONTEXT.sys_post_service.remove(&dict_id).await;
    RespVO::from_result(&data).resp_json()
}

// 岗位下拉框（对应Spring GET /system/post/optionselect）
#[get("/post/optionselect")]
pub async fn optionselect() -> impl Responder {
    let data = CONTEXT.sys_post_service.finds_all().await;
    RespVO::from_result(&data).resp_json()
}

// 岗位管理-部门树（对应Spring GET /system/post/deptTree）
#[get("/post/deptTree")]
pub async fn dept_tree(current_user: CurrentUser) -> impl Responder {
    let data = CONTEXT.sys_dept_service.get_dept_tree(&current_user.name).await;
    RespVO::from_result(&data).resp_json()
}

// 数据导出（对应Spring POST /post/export）——分批拉取，避免全量进内存
#[post("/post/export")]
pub async fn export(query: web::Query<PostPageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["岗位编号", "岗位名称", "岗位编码", "岗位排序", "状态", "备注"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_post_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|p| vec![
                p.post_id.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                p.post_name.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                p.post_code.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                p.post_sort.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                p.status.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                p.remark.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("导出数据", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "post.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
