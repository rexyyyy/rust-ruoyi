use crate::modules::system::domain::bo::dict_type::{DictTypeAddDTO, DictTypeUpdateDTO, DictTypePageDTO};
use crate::web_data::CurrentUser;
use crate::modules::system::domain::SysDictType;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::CONTEXT;
use futures_util::future::BoxFuture;
use actix_web::{get, post, put, web, Responder, delete};

#[get("/dict/type/list")]
pub async fn page(page: web::Query<DictTypePageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_type_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/dict/type/optionselect")]
pub async fn optionselect() -> impl Responder {
    let data = CONTEXT.sys_dict_type_service.finds_all().await;
    RespVO::from_result(&data).resp_json()
}

#[get("/dict/type/{dict_id}")]
pub async fn detail(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let dict_type_vo = CONTEXT.sys_dict_type_service.detail(&dict_id).await;
    RespVO::from_result(&dict_type_vo).resp_json()
}

#[post("/dict/type")]
pub async fn add(current_user: CurrentUser, arg: web::Json<DictTypeAddDTO>) -> impl Responder {
    let mut data = SysDictType::from(arg.0);
    data.create_by = Some(current_user.id);
    if data.dict_name.is_none() {
        return RespVO::<u64>::from_error_info(500, "字典名字不能为空!").resp_json();
    }
    let data = CONTEXT.sys_dict_type_service.add(&data).await;
    RespVO::from_result(&data).resp_json()
}

#[put("/dict/type")]
pub async fn update(current_user: CurrentUser, arg: web::Json<DictTypeUpdateDTO>) -> impl Responder {
    let mut data = SysDictType::from(arg.0);
    data.update_by = Some(current_user.id);
    let data = CONTEXT.sys_dict_type_service.update(data).await;
    RespVO::from_result(&data).resp_json()
}

#[delete("/dict/type/{dict_id}")]
pub async fn remove(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let data = CONTEXT.sys_dict_type_service
        .remove(&dict_id).await;
    RespVO::from_result(&data).resp_json()
}

// 刷新字典缓存（对应Spring DELETE /system/dict/type/refreshCache）
#[delete("/dict/type/refreshCache")]
pub async fn refresh_cache() -> impl Responder {
    CONTEXT.sys_dict_data_service.update_cache().await;
    RespVO::<String>::from_success_info("刷新成功！").resp_json()
}

// 数据导出（对应Spring POST /dict/type/export）——分批拉取，避免全量进内存
#[post("/dict/type/export")]
pub async fn export(query: web::Query<DictTypePageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["字典主键", "字典名称", "字典类型", "状态", "备注"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_dict_type_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|d| vec![
                d.dict_id.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                d.dict_name.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                d.dict_type.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                String::new(),
                d.remark.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("导出数据", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "dictType.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
