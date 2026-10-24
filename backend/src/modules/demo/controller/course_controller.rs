//! DemoCourse 控制器
//! 源表：demo_course　业务：演示课程表
//! 由 codegen 于 2026-10-08 21:03:18 生成。
//!
//! 注册方式见同级 `mod.rs` 的 `init()`；本文件的路由挂载在
//! `http_server.rs` 的 `web::scope("/demo")` 之下。

use actix_web::{delete, get, post, put, web, Responder};
use futures_util::future::BoxFuture;

use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::web_data::CurrentUser;
use crate::modules::demo::domain::bo::{
    DemoCourseAddDTO, DemoCoursePageDTO, DemoCourseUpdateDTO,
};
use crate::modules::demo::domain::entity::DemoCourse;
use crate::modules::demo::service::DemoCourseService;

fn service() -> DemoCourseService {
    DemoCourseService {}
}

/// 分页列表
#[get("/course/list")]
pub async fn page(query: web::Query<DemoCoursePageDTO>) -> impl Responder {
    let data = service().page(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 详情
#[get("/course/{course_id}")]
pub async fn detail(id: web::Path<String>) -> impl Responder {
    let vo = service().detail(&id.into_inner()).await;
    RespVO::from_result(&vo).resp_json()
}

/// 新增
#[post("/course")]
pub async fn add(current_user: CurrentUser, arg: web::Json<DemoCourseAddDTO>) -> impl Responder {
    let mut data = DemoCourse::from(arg.0);
    data.create_by = Some(current_user.id);
    let affected = match service().add(&data).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "新增失败！".to_string()).resp_json()
}

/// 修改
#[put("/course")]
pub async fn update(current_user: CurrentUser, arg: web::Json<DemoCourseUpdateDTO>) -> impl Responder {
    let mut data = DemoCourse::from(arg.0);
    data.update_by = Some(current_user.id);
    let affected = match service().update(data).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "修改失败！".to_string()).resp_json()
}

/// 删除
#[delete("/course/{course_id}")]
pub async fn remove(id: web::Path<String>) -> impl Responder {
    let affected = match service().remove(&id.into_inner()).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "删除失败！".to_string()).resp_json()
}

/// 导出（分批拉取，避免全量进内存）
#[post("/course/export")]
pub async fn export(query: web::Query<DemoCoursePageDTO>) -> impl Responder {
    let base = query.0.clone();
    let headers = ["课程编号", "课程编码", "课程名称", "授课教师", "学分", "开课状态", "开课日期", "备注", "创建者", "创建时间", "更新者", "更新时间"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = service().page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|r| vec![
                r.courseId.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.courseCode.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.courseName.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.teacher.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.credit.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.openStatus.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.startDate.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.remark.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.createBy.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.createTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.updateBy.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.updateTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("演示课程表", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "course.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
