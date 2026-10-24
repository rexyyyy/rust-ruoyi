//! OmsRegorder 控制器
//! 源表：oms_regorder　业务：注册码登记
//! 由 codegen 于 2026-10-10 16:24:42 生成。
//!
//! 注册方式见同级 `mod.rs` 的 `init()`；本文件的路由挂载在
//! `http_server.rs` 的 `web::scope("/oms")` 之下。

use actix_web::{delete, get, post, put, web, Responder};

use crate::modules::system::domain::vo::{PageVO, RespVO};
use crate::web_data::CurrentUser;
use crate::modules::oms::domain::bo::{
    OmsRegorderAddDTO, OmsRegorderPageDTO, OmsRegorderUpdateDTO,
};
use crate::modules::oms::domain::entity::OmsRegorder;
use crate::modules::oms::service::OmsRegorderService;

fn service() -> OmsRegorderService {
    OmsRegorderService {}
}

/// 分页列表
#[get("/regorder/list")]
pub async fn page(query: web::Query<OmsRegorderPageDTO>) -> impl Responder {
    let data = service().page(&query).await;
    PageVO::from_result(&data).resp_json()
}

/// 详情
#[get("/regorder/{regid}")]
pub async fn detail(id: web::Path<String>) -> impl Responder {
    let vo = service().detail(&id.into_inner()).await;
    RespVO::from_result(&vo).resp_json()
}

/// 新增
#[post("/regorder")]
pub async fn add(current_user: CurrentUser, arg: web::Json<OmsRegorderAddDTO>) -> impl Responder {
    let mut data = OmsRegorder::from(arg.0);
    // 根据需求可以使给主键填业务流水号（按日期前缀取最大号 +1），放开下面两行即可：
    let a = crate::modules::system::service::ServiceContext::get_table_maxid("regid", "oms_regorder").await ;
       data.regid = a;
    data.create_by = Some(current_user.name.clone());
    let affected = match service().add(&data).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "新增失败！".to_string()).resp_json()
}

/// 修改
#[put("/regorder")]
pub async fn update(current_user: CurrentUser, arg: web::Json<OmsRegorderUpdateDTO>) -> impl Responder {
    let mut data = OmsRegorder::from(arg.0);
    data.update_by = Some(current_user.name.clone());
    let affected = match service().update(data).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "修改失败！".to_string()).resp_json()
}

/// 删除
#[delete("/regorder/{regid}")]
pub async fn remove(id: web::Path<String>) -> impl Responder {
    let affected = match service().remove(&id.into_inner()).await {
        Ok(v) => v,
        Err(e) => return RespVO::<u64>::from_error_info(500, &e.to_string()).resp_json(),
    };
    RespVO::<u64>::judge(affected, String::new(), "删除失败！".to_string()).resp_json()
}

/// 导出
#[post("/regorder/export")]
pub async fn export(query: web::Query<OmsRegorderPageDTO>) -> impl Responder {
    let mut dto = query.0.clone();
    dto.page_size = Some(10000);
    let data = service().page(&dto).await;
    let rows = data.map(|p| p.records).unwrap_or_default();
    let headers = ["主键", "订单号", "用户名", "订单总金额", "应付金额", "支付方式：0->未支付；1->支付宝；2->微信", "订单来源：0->PC订单；1->app订单", "订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单", "订单类型：0->正常订单；1->秒杀订单", "物流公司(配送方式)", "申请号", "回复号", "收货人姓名", "收货人电话", "省份/直辖市", "城市", "区", "订单备注", "确认收货状态：0->未确认；1->已确认", "支付时间", "发货时间", "确认收货时间", "评价时间", "删除标志（0代表存在 2代表删除）", "创建者", "创建时间", "更新者", "更新时间"];
    let mut body: Vec<Vec<String>> = Vec::with_capacity(rows.len());
    for r in &rows {
        body.push(vec![
            r.regid.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.orderSn.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.memberUsername.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.totalAmount.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.payAmount.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.payType.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.sourceType.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.status.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.orderType.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.deliveryCompany.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.requestSn.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.deliverySn.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.receiverName.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.receiverPhone.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.receiverProvince.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.receiverCity.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.receiverRegion.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.note.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.confirmStatus.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.paymentTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.deliveryTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.receiveTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.commentTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.delFlag.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.createBy.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.createTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.updateBy.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            r.updateTime.as_ref().map(|v| v.to_string()).unwrap_or_default(),
        ]);
    }
    match crate::util::excel::build_xlsx("注册码登记", &headers, &body) {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "regorder.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
