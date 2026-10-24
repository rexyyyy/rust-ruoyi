use actix_web::{delete, get, post,   Responder, web};

use crate::modules::system::domain::bo::logininfor::LogininforPageDTO;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::CONTEXT;
use futures_util::future::BoxFuture;

#[get("/logininfor/list")]
pub async fn page(page: web::Query<LogininforPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_logininfor_service.page(&page.0).await;
    
    PageVO::from_result(&data).resp_json()
}


#[delete("/logininfor/{info_id}")]
pub async fn remove(info_id: web::Path<String>) -> impl Responder {
    let info_id = info_id.into_inner();
    let data = CONTEXT
        .sys_logininfor_service
        .remove(&info_id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}

// 清空登录日志（对应Spring DELETE /monitor/logininfor/clean）
#[delete("/logininfor/clean")]
pub async fn clean() -> impl Responder {
    let r = CONTEXT.sys_logininfor_service.clean().await;
    RespVO::from_result(&r).resp_json()
}

// 解锁用户登录状态（对应Spring GET /monitor/logininfor/unlock/{userName}）
#[get("/logininfor/unlock/{user_name}")]
pub async fn unlock(user_name: web::Path<String>) -> impl Responder {
    let user_name = user_name.into_inner();
    CONTEXT.cache_service.del(&format!("pwd_err_cnt:{}", user_name)).await;
    RespVO::<String>::from_success_info("解锁成功！").resp_json()
}

// 数据导出（对应Spring POST /logininfor/export）——分批拉取，避免全量进内存
#[post("/logininfor/export")]
pub async fn export(query: web::Query<LogininforPageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["访问编号", "用户名称", "登录地址", "登录地点", "浏览器", "操作系统", "登录状态", "操作信息", "登录日期"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_logininfor_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|l| vec![
                l.info_id.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.user_name.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.ipaddr.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.login_location.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.browser.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.os.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.status.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.msg.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                l.login_time.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("导出数据", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "logininfor.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
