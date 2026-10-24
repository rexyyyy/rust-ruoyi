//! OmsRegorder 返回对象（VO）
//! 源表：oms_regorder　业务：注册码登记
//! 由 codegen 于 2026-10-10 16:24:42 生成。
//!
//! VO 与前端 TS 类型（`types.ts`）一一对应：`rename_all = "camelCase"`
//! 保证 JSON 字段名就是 Vue 侧的属性名。

use serde::{Deserialize, Serialize};

use crate::modules::oms::domain::entity::OmsRegorder;

#[allow(non_snake_case)] // VO 字段即前端 camelCase 属性名，有意为之
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OmsRegorderVO {
    // 主键统一输出成字符串：PG bigint 的精度超过 JS Number 安全范围，
    // 前端直接塞进 URL / 表单若经过 JSON 数字会有精度丢失风险。
    pub regid: Option<String>,
    pub orderSn: Option<String>, // 订单号
    pub memberUsername: Option<String>, // 用户名
    pub totalAmount: Option<f64>, // 订单总金额
    pub payAmount: Option<f64>, // 应付金额
    pub payType: Option<String>, // 支付方式：0->未支付；1->支付宝；2->微信
    pub sourceType: Option<String>, // 订单来源：0->PC订单；1->app订单
    pub status: Option<String>, // 订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单
    pub orderType: Option<String>, // 订单类型：0->正常订单；1->秒杀订单
    pub deliveryCompany: Option<String>, // 物流公司(配送方式)
    pub requestSn: Option<String>, // 申请号
    pub deliverySn: Option<String>, // 回复号
    pub receiverName: Option<String>, // 收货人姓名
    pub receiverPhone: Option<String>, // 收货人电话
    pub receiverProvince: Option<String>, // 省份/直辖市
    pub receiverCity: Option<String>, // 城市
    pub receiverRegion: Option<String>, // 区
    pub note: Option<String>, // 订单备注
    pub confirmStatus: Option<String>, // 确认收货状态：0->未确认；1->已确认
    pub paymentTime: Option<chrono::NaiveDateTime>, // 支付时间
    pub deliveryTime: Option<chrono::NaiveDateTime>, // 发货时间
    pub receiveTime: Option<chrono::NaiveDateTime>, // 确认收货时间
    pub commentTime: Option<chrono::NaiveDateTime>, // 评价时间
    pub delFlag: Option<String>, // 删除标志（0代表存在 2代表删除）
    pub createBy: Option<String>, // 创建者
    pub createTime: Option<chrono::NaiveDateTime>, // 创建时间
    pub updateBy: Option<String>, // 更新者
    pub updateTime: Option<chrono::NaiveDateTime>, // 更新时间
}

impl From<OmsRegorder> for OmsRegorderVO {
    fn from(arg: OmsRegorder) -> Self {
        Self {
            regid: Some(arg.regid.to_string()),
            orderSn: arg.order_sn,
            memberUsername: arg.member_username,
            totalAmount: arg.total_amount,
            payAmount: arg.pay_amount,
            payType: arg.pay_type,
            sourceType: arg.source_type,
            status: arg.status,
            orderType: arg.order_type,
            deliveryCompany: arg.delivery_company,
            requestSn: arg.request_sn,
            deliverySn: arg.delivery_sn,
            receiverName: arg.receiver_name,
            receiverPhone: arg.receiver_phone,
            receiverProvince: arg.receiver_province,
            receiverCity: arg.receiver_city,
            receiverRegion: arg.receiver_region,
            note: arg.note,
            confirmStatus: arg.confirm_status,
            paymentTime: arg.payment_time,
            deliveryTime: arg.delivery_time,
            receiveTime: arg.receive_time,
            commentTime: arg.comment_time,
            delFlag: arg.del_flag,
            createBy: arg.create_by,
            createTime: arg.create_time,
            updateBy: arg.update_by,
            updateTime: arg.update_time,
        }
    }
}
