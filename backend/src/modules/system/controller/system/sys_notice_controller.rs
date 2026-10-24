use crate::modules::system::domain::bo::notice::{NoticeAddDTO, NoticeUpdateDTO, NoticePageDTO};
use crate::modules::system::domain::SysNotice;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::CONTEXT;
use crate::web_data::CurrentUser;
use actix_web::{get, post, put, delete, web, Responder};

#[get("/notice/list")]
pub async fn page(page: web::Query<NoticePageDTO>) -> impl Responder {
    let data = CONTEXT.sys_notice_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/notice/{notice_id}")]
pub async fn detail(notice_id: web::Path<String>) -> impl Responder {
    let notice_id = notice_id.into_inner();
    let notice_vo = CONTEXT.sys_notice_service.detail(&notice_id).await;
    RespVO::from_result(&notice_vo).resp_json()
}


#[post("/notice")]
pub async fn add(current_user: CurrentUser, arg: web::Json<NoticeAddDTO>) -> impl Responder {
    let mut data = SysNotice::from(arg.0);
    data.create_by = Some(current_user.id);
    let res = SysNotice::from(data);
    let data = CONTEXT.sys_notice_service.add(&res).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/notice")]
pub async fn update(current_user: CurrentUser, arg: web::Json<NoticeUpdateDTO>) -> impl Responder {
    let mut data = SysNotice::from(arg.0);
    data.update_by = Some(current_user.id);
    let data = CONTEXT.sys_notice_service.update(data).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/notice/{notice_id}")]
pub async fn remove(notice_id: web::Path<String>) -> impl Responder {
    let notice_id = notice_id.into_inner();
    let data = CONTEXT
        .sys_notice_service
        .remove(&notice_id)
        .await;
    RespVO::<u64>::judge(data.unwrap(), "".to_string(), "删除失败！".to_string()).resp_json()
}
