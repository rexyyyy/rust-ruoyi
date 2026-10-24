use actix_web::{delete, get, post, Responder, web};

use crate::modules::system::domain::bo::operlog::OperLogPageDTO;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::CONTEXT;
use crate::util::excel::{build_xlsx_streaming, excel_response};
use futures_util::future::BoxFuture;

// 操作日志列表（对应Spring GET /monitor/operlog/list）
#[get("/operlog/list")]
pub async fn page(page: web::Query<OperLogPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_operlog_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}

// 操作日志导出（对应Spring POST /monitor/operlog/export）——分批拉取，避免全量进内存
#[post("/operlog/export")]
pub async fn export(query: web::Query<OperLogPageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["日志主键", "模块标题", "业务类型", "方法名称", "请求方式", "操作人员", "部门名称", "请求URL", "操作IP", "操作地点", "操作参数", "返回参数", "状态", "错误消息", "操作时间"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_operlog_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|r| vec![
                r.oper_id.clone().unwrap_or_default(),
                r.title.clone().unwrap_or_default(),
                r.business_type.map(|v| v.to_string()).unwrap_or_default(),
                r.method.clone().unwrap_or_default(),
                r.request_method.clone().unwrap_or_default(),
                r.oper_name.clone().unwrap_or_default(),
                r.dept_name.clone().unwrap_or_default(),
                r.oper_url.clone().unwrap_or_default(),
                r.oper_ip.clone().unwrap_or_default(),
                r.oper_location.clone().unwrap_or_default(),
                r.oper_param.clone().unwrap_or_default(),
                r.json_result.clone().unwrap_or_default(),
                r.status.map(|v| v.to_string()).unwrap_or_default(),
                r.error_msg.clone().unwrap_or_default(),
                r.oper_time.as_ref().map(|d| d.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match build_xlsx_streaming("操作日志", &headers, 500, fetch).await {
        Ok(bytes) => excel_response(bytes, "operlog.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}

// 删除操作日志（对应Spring DELETE /monitor/operlog/{operIds}）
#[delete("/operlog/{oper_ids}")]
pub async fn remove(oper_ids: web::Path<String>) -> impl Responder {
    let oper_ids = oper_ids.into_inner();
    let ids: Vec<String> = oper_ids.split(",").map(|s| s.to_string()).collect();
    let mut affected = 0u64;
    for id in ids {
        let vo = CONTEXT.sys_operlog_service.remove(&id).await;
        match vo {
            Ok(n) => affected += n,
            Err(e) => return RespVO::from_error_result(500, &Err::<u64, _>(e)).resp_json(),
        }
    }
    RespVO::from_result(&Ok(affected)).resp_json()
}

// 清空操作日志（对应Spring DELETE /monitor/operlog/clean）
#[delete("/operlog/clean")]
pub async fn clean() -> impl Responder {
    let r = CONTEXT.sys_operlog_service.clean().await;
    RespVO::from_result(&r).resp_json()
}
