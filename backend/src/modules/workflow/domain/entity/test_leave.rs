//! 请假单 test_leave（工作流接入示例，对标 Java `TestLeave`）。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "test_leave")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 申请编号（流程开启后回填业务编号）
    pub apply_code: Option<String>,
    pub leave_type: String,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub leave_days: i32,
    pub remark: Option<String>,
    /// 业务状态，取值见 `service::biz::BusinessStatus`
    pub status: Option<String>,
    pub create_by: Option<i64>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<i64>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
