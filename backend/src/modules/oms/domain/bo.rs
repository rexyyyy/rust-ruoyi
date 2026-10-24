//! OmsRegorder 请求对象（DTO）
//! 源表：oms_regorder　业务：注册码登记
//! 由 codegen 于 2026-10-10 16:24:42 生成。

use serde::{Deserialize, Serialize};

use crate::modules::system::domain::PageRequest;
use crate::modules::oms::domain::entity::OmsRegorder;

/// 分页查询参数
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OmsRegorderPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub member_username: Option<String>, // 用户名
    pub pay_type: Option<String>, // 支付方式：0->未支付；1->支付宝；2->微信
    pub source_type: Option<String>, // 订单来源：0->PC订单；1->app订单
    pub status: Option<String>, // 订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单
    pub order_type: Option<String>, // 订单类型：0->正常订单；1->秒杀订单
    pub receiver_name: Option<String>, // 收货人姓名
}

impl From<OmsRegorderPageDTO> for PageRequest {
    fn from(arg: OmsRegorderPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&OmsRegorderPageDTO> for PageRequest {
    fn from(arg: &OmsRegorderPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

/// 新增参数
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OmsRegorderAddDTO {
    pub order_sn: Option<String>, // 订单号
    pub member_username: Option<String>, // 用户名
    pub total_amount: Option<f64>, // 订单总金额
    pub pay_amount: Option<f64>, // 应付金额
    pub pay_type: Option<String>, // 支付方式：0->未支付；1->支付宝；2->微信
    pub source_type: Option<String>, // 订单来源：0->PC订单；1->app订单
    pub status: Option<String>, // 订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单
    pub order_type: Option<String>, // 订单类型：0->正常订单；1->秒杀订单
    pub delivery_company: Option<String>, // 物流公司(配送方式)
    pub request_sn: Option<String>, // 申请号
    pub delivery_sn: Option<String>, // 回复号
    pub receiver_name: Option<String>, // 收货人姓名
    pub receiver_phone: Option<String>, // 收货人电话
    pub receiver_province: Option<String>, // 省份/直辖市
    pub receiver_city: Option<String>, // 城市
    pub receiver_region: Option<String>, // 区
    pub note: Option<String>, // 订单备注
    pub confirm_status: Option<String>, // 确认收货状态：0->未确认；1->已确认
    pub payment_time: Option<chrono::NaiveDateTime>, // 支付时间
    pub delivery_time: Option<chrono::NaiveDateTime>, // 发货时间
    pub receive_time: Option<chrono::NaiveDateTime>, // 确认收货时间
    pub comment_time: Option<chrono::NaiveDateTime>, // 评价时间
    pub del_flag: Option<String>, // 删除标志（0代表存在 2代表删除）
}

impl From<OmsRegorderAddDTO> for OmsRegorder {
    fn from(arg: OmsRegorderAddDTO) -> Self {
        Self {
            regid: crate::util::idseq::next_id(),
            order_sn: arg.order_sn,
            member_username: arg.member_username,
            total_amount: arg.total_amount,
            pay_amount: arg.pay_amount,
            pay_type: arg.pay_type,
            source_type: arg.source_type,
            status: arg.status,
            order_type: arg.order_type,
            delivery_company: arg.delivery_company,
            request_sn: arg.request_sn,
            delivery_sn: arg.delivery_sn,
            receiver_name: arg.receiver_name,
            receiver_phone: arg.receiver_phone,
            receiver_province: arg.receiver_province,
            receiver_city: arg.receiver_city,
            receiver_region: arg.receiver_region,
            note: arg.note,
            confirm_status: arg.confirm_status,
            payment_time: arg.payment_time,
            delivery_time: arg.delivery_time,
            receive_time: arg.receive_time,
            comment_time: arg.comment_time,
            del_flag: arg.del_flag,
            create_by: None,
            create_time: Some(chrono::Local::now().naive_local()),
            update_by: None,
            update_time: None,
        }
    }
}

/// 兼容前端回传的雪花 ID：字符串 / 整数 / 浮点（JS Number）/ null 四种 JSON 形态均可
fn string_or_number<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        S(String),
        I(i64),
        F(f64),
    }
    match Option::<V>::deserialize(deserializer)? {
        None => Ok(None),
        Some(V::S(s)) => s.parse().map(Some).map_err(serde::de::Error::custom),
        Some(V::I(i)) => Ok(Some(i)),
        Some(V::F(f)) => Ok(Some(f as i64)),
    }
}

/// 修改参数
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OmsRegorderUpdateDTO {
    // VO 主键输出为 String（防 JS 精度丢失），前端会原样回传，
    // 因此主键同时接受字符串与数字两种 JSON 形态
    #[serde(deserialize_with = "string_or_number")]
    pub regid: Option<i64>,
    pub order_sn: Option<String>, // 订单号
    pub member_username: Option<String>, // 用户名
    pub total_amount: Option<f64>, // 订单总金额
    pub pay_amount: Option<f64>, // 应付金额
    pub pay_type: Option<String>, // 支付方式：0->未支付；1->支付宝；2->微信
    pub source_type: Option<String>, // 订单来源：0->PC订单；1->app订单
    pub status: Option<String>, // 订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单
    pub order_type: Option<String>, // 订单类型：0->正常订单；1->秒杀订单
    pub delivery_company: Option<String>, // 物流公司(配送方式)
    pub request_sn: Option<String>, // 申请号
    pub delivery_sn: Option<String>, // 回复号
    pub receiver_name: Option<String>, // 收货人姓名
    pub receiver_phone: Option<String>, // 收货人电话
    pub receiver_province: Option<String>, // 省份/直辖市
    pub receiver_city: Option<String>, // 城市
    pub receiver_region: Option<String>, // 区
    pub note: Option<String>, // 订单备注
    pub confirm_status: Option<String>, // 确认收货状态：0->未确认；1->已确认
    pub payment_time: Option<chrono::NaiveDateTime>, // 支付时间
    pub delivery_time: Option<chrono::NaiveDateTime>, // 发货时间
    pub receive_time: Option<chrono::NaiveDateTime>, // 确认收货时间
    pub comment_time: Option<chrono::NaiveDateTime>, // 评价时间
    pub del_flag: Option<String>, // 删除标志（0代表存在 2代表删除）
}

impl From<OmsRegorderUpdateDTO> for OmsRegorder {
    fn from(arg: OmsRegorderUpdateDTO) -> Self {
        Self {
            regid: arg.regid.unwrap_or_default(),
            order_sn: arg.order_sn,
            member_username: arg.member_username,
            total_amount: arg.total_amount,
            pay_amount: arg.pay_amount,
            pay_type: arg.pay_type,
            source_type: arg.source_type,
            status: arg.status,
            order_type: arg.order_type,
            delivery_company: arg.delivery_company,
            request_sn: arg.request_sn,
            delivery_sn: arg.delivery_sn,
            receiver_name: arg.receiver_name,
            receiver_phone: arg.receiver_phone,
            receiver_province: arg.receiver_province,
            receiver_city: arg.receiver_city,
            receiver_region: arg.receiver_region,
            note: arg.note,
            confirm_status: arg.confirm_status,
            payment_time: arg.payment_time,
            delivery_time: arg.delivery_time,
            receive_time: arg.receive_time,
            comment_time: arg.comment_time,
            del_flag: arg.del_flag,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: Some(chrono::Local::now().naive_local()),
        }
    }
}
