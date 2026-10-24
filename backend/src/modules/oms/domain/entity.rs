//! OmsRegorder sea-orm 实体
//! 源表：oms_regorder（注册码登记）
//!
//! 由 codegen 于 2026-10-10 16:24:42 生成，重新生成时本文件会被覆盖。
//!
//! 说明：非原生类型统一写成全限定路径（`chrono::NaiveDateTime` 等），
//! 因此除 prelude 外不需要任何额外 `use`，也就不存在多余的未使用导入。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "oms_regorder")]
pub struct Model {
    // 主键在 DTO 层显式赋值（雪花 ID / uuid），故一律 auto_increment = false
    #[sea_orm(primary_key, auto_increment = false)]
    pub regid: i64,
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
    pub create_by: Option<String>, // 创建者
    pub create_time: Option<chrono::NaiveDateTime>, // 创建时间
    pub update_by: Option<String>, // 更新者
    pub update_time: Option<chrono::NaiveDateTime>, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// 别名沿用 backend 既有模块（sys_post 等）的书写习惯
pub use Model as OmsRegorder;
pub use Entity as OmsRegorderEntity;
pub use ActiveModel as OmsRegorderActiveModel;
pub use Column as OmsRegorderColumn;
