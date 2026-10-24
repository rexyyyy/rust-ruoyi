use crate::modules::system::domain::bo::dict_data::{DictDataAddDTO, DictDataUpdateDTO, DictDataPageDTO};
use crate::web_data::CurrentUser;
use crate::modules::system::domain::SysDictData;
use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::modules::system::service::CONTEXT;
use futures_util::future::BoxFuture;
use actix_web::{get, post, put, delete, web, Responder};

#[get("/dict/data/list")]
pub async fn page(page: web::Query<DictDataPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_data_service.page(&page.0).await;
    PageVO::from_result(&data).resp_json()
}


#[get("/dict/data/{dict_id}")]
pub async fn detail(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let dict_data_vo = CONTEXT.sys_dict_data_service.detail(&dict_id).await;
    RespVO::from_result(&dict_data_vo).resp_json()
}

#[post("/dict/data")]
pub async fn add(current_user: CurrentUser, arg: web::Json<DictDataAddDTO>) -> impl Responder {
    let mut data = SysDictData::from(arg.0);
    data.create_by = Some(current_user.id);
    let data = CONTEXT.sys_dict_data_service.add(&data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "添加失败！".to_string()).resp_json()
}

#[put("/dict/data")]
pub async fn update(current_user: CurrentUser, arg: web::Json<DictDataUpdateDTO>) -> impl Responder {
    let mut data = SysDictData::from(arg.0);
    data.update_by = Some(current_user.id);
    let data = CONTEXT.sys_dict_data_service.update(data).await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "更新失败！".to_string()).resp_json()
}

#[delete("/dict/data/{dict_code}")]
pub async fn remove(dict_id: web::Path<String>) -> impl Responder {
    let dict_id = dict_id.into_inner();
    let data = CONTEXT
        .sys_dict_data_service
        .remove(&dict_id)
        .await.unwrap();
    RespVO::<u64>::judge(data, "".to_string(), "删除失败！".to_string()).resp_json()
}


#[get("/dict/data/type/{dict_type}")]
pub async fn get_by_dict_type(dict_type: web::Path<String>) -> impl Responder {
    let dict_type = dict_type.into_inner();
    let dict_data_vo = CONTEXT.sys_dict_data_service.get_by_dict_type(&dict_type).await;
    RespVO::from_result(&dict_data_vo).resp_json()
}

// 数据导出（对应Spring POST /dict/data/export）——分批拉取，避免全量进内存
#[post("/dict/data/export")]
pub async fn export(query: web::Query<DictDataPageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["字典编码", "字典排序", "字典标签", "字典键值", "字典类型", "状态", "备注"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_dict_data_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|d| vec![
                d.dict_code.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                d.dict_sort.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                d.dict_label.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                d.dict_value.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                d.dict_type.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                String::new(),
                d.remark.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("导出数据", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "dictData.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
