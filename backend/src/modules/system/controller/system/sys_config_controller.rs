use crate::modules::system::domain::bo::config::{ConfigAddDTO, ConfigUpdateDTO, ConfigPageDTO};
use crate::web_data::CurrentUser;
use crate::modules::system::domain::SysConfig;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::CONTEXT;
use futures_util::future::BoxFuture;
use crate::modules::system::domain::bo::config::ConfigUpdateByKeyDTO;
use crate::modules::system::domain::vo::RespJson;
use actix_web::{get, post, put, delete, web, Responder};

#[get("/config/list")]
pub async fn page(page: web::Query<ConfigPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_config_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/config/{config_id}")]
pub async fn detail(config_id: web::Path<String>) -> impl Responder {
    let config_id = config_id.into_inner();
    let config_vo = CONTEXT.sys_config_service.detail(&config_id).await;
    RespVO::from_result(&config_vo).resp_json()
}


#[post("/config")]
pub async fn add(current_user: CurrentUser, arg: web::Json<ConfigAddDTO>) -> impl Responder {
    let mut data = SysConfig::from(arg.0);
    data.create_by = Some(current_user.id);
    let res = SysConfig::from(data);
    let data = CONTEXT.sys_config_service.add(&res).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/config")]
pub async fn update(current_user: CurrentUser, arg: web::Json<ConfigUpdateDTO>) -> impl Responder {
    let mut data = SysConfig::from(arg.0);
    data.update_by = Some(current_user.id);
    let data = CONTEXT.sys_config_service.update(data).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/config/{config_id}")]
pub async fn remove(config_id: web::Path<String>) -> impl Responder {
    let config_id = config_id.into_inner();
    let data = CONTEXT
        .sys_config_service
        .remove(&config_id)
        .await;
    RespVO::<u64>::judge(data.unwrap(), "".to_string(), "删除失败！".to_string()).resp_json()
}

// 根据参数键名查询参数值（对应Spring GET /system/config/configKey/{configKey}）
#[get("/config/configKey/{config_key}")]
pub async fn get_config_key(config_key: web::Path<String>) -> impl Responder {
    let config_key = config_key.into_inner();
    let value = CONTEXT.sys_config_service.select_config_by_key(&config_key).await.unwrap_or_default();
    let mut res = RespJson::success_info("操作成功");
    // 官方 R<String>：data 就是参数值本身（前端 response.data 当字符串用）
    res.insert("data".to_string(), serde_json::json!(value));
    res.resp_json()
}

// 根据参数键名修改参数值（对应Spring PUT /system/config/updateByKey）
#[put("/config/updateByKey")]
pub async fn update_by_key(arg: web::Json<ConfigUpdateByKeyDTO>) -> impl Responder {
    let dto = arg.0;
    let config_key = dto.config_key.unwrap_or_default();
    let config_value = dto.config_value.unwrap_or_default();
    if config_key.is_empty() {
        return RespVO::<u64>::from_error_info(500, "参数键名不能为空").resp_json();
    }
    let r = CONTEXT.sys_config_service.update_by_key(&config_key, &config_value).await;
    RespVO::from_result(&r).resp_json()
}

// 刷新参数缓存（对应Spring DELETE /system/config/refreshCache）
#[delete("/config/refreshCache")]
pub async fn refresh_cache() -> impl Responder {
    CONTEXT.sys_config_service.reset_config_cache().await;
    RespVO::<String>::from_success_info("刷新成功！").resp_json()
}

// 数据导出（对应Spring POST /config/export）——分批拉取，避免全量进内存
#[post("/config/export")]
pub async fn export(query: web::Query<ConfigPageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["参数主键", "参数名称", "参数键名", "参数键值", "系统内置", "备注"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_config_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|c| vec![
                c.config_id.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                c.config_name.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                c.config_key.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                c.config_value.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                c.config_type.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                c.remark.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("导出数据", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "config.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
