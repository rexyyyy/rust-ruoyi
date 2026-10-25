//! 流程分类 flow_category（对标 Java `FlowCategory`）。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "flow_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i64,
    /// 父分类 id，根节点为 0
    pub parent_id: i64,
    /// 祖级列表，如 `0,100,101`
    pub ancestors: Option<String>,
    pub category_name: String,
    pub order_num: Option<i32>,
    pub del_flag: Option<String>,
    pub create_by: Option<i64>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<i64>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
