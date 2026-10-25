//! SeaORM 实体。表结构与 Java 版 `sql/*/warm-flow-all.sql` 一一对应。
//!
//! 只做持久化映射，字段语义见 `warm-flow-core` 的 `entity` 模块。
//! 表名与列名保持原样，方便与已有数据库共用。

use sea_orm::entity::prelude::*;

/// 流程定义 flow_definition
pub mod flow_definition {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_definition")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub flow_code: String,
        pub flow_name: String,
        pub model_value: String,
        pub category: Option<String>,
        pub version: String,
        pub is_publish: i32,
        pub form_custom: Option<String>,
        pub form_path: Option<String>,
        pub activity_status: i32,
        pub listener_type: Option<String>,
        pub listener_path: Option<String>,
        pub ext: Option<String>,
        pub create_time: Option<DateTime>,
        pub create_by: Option<String>,
        pub update_time: Option<DateTime>,
        pub update_by: Option<String>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// 流程节点 flow_node
pub mod flow_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_node")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub node_type: i32,
        pub definition_id: i64,
        pub node_code: String,
        pub node_name: Option<String>,
        pub permission_flag: Option<String>,
        pub node_ratio: Option<String>,
        pub coordinate: Option<String>,
        pub any_node_skip: Option<String>,
        pub listener_type: Option<String>,
        pub listener_path: Option<String>,
        pub form_custom: Option<String>,
        pub form_path: Option<String>,
        pub version: Option<String>,
        pub create_time: Option<DateTime>,
        pub create_by: Option<String>,
        pub update_time: Option<DateTime>,
        pub update_by: Option<String>,
        pub ext: Option<String>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// 节点跳转 flow_skip
pub mod flow_skip {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_skip")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub definition_id: i64,
        pub now_node_code: String,
        pub now_node_type: Option<i32>,
        pub next_node_code: String,
        pub next_node_type: Option<i32>,
        pub skip_name: Option<String>,
        pub skip_type: Option<String>,
        pub skip_condition: Option<String>,
        pub coordinate: Option<String>,
        pub create_time: Option<DateTime>,
        pub create_by: Option<String>,
        pub update_time: Option<DateTime>,
        pub update_by: Option<String>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// 流程实例 flow_instance
pub mod flow_instance {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_instance")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub definition_id: i64,
        pub business_id: String,
        pub node_type: i32,
        pub node_code: String,
        pub node_name: Option<String>,
        pub variable: Option<String>,
        pub flow_status: String,
        pub activity_status: i32,
        pub def_json: Option<String>,
        pub create_time: Option<DateTime>,
        pub create_by: Option<String>,
        pub update_time: Option<DateTime>,
        pub update_by: Option<String>,
        pub ext: Option<String>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// 待办任务 flow_task
pub mod flow_task {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_task")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub definition_id: i64,
        pub instance_id: i64,
        pub node_code: String,
        pub node_name: Option<String>,
        pub node_type: i32,
        pub flow_status: String,
        pub form_custom: Option<String>,
        pub form_path: Option<String>,
        pub create_time: Option<DateTime>,
        pub create_by: Option<String>,
        pub update_time: Option<DateTime>,
        pub update_by: Option<String>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// 历史任务 flow_his_task
pub mod flow_his_task {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_his_task")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub definition_id: i64,
        pub instance_id: i64,
        pub task_id: i64,
        pub node_code: Option<String>,
        pub node_name: Option<String>,
        pub node_type: Option<i32>,
        pub target_node_code: Option<String>,
        pub target_node_name: Option<String>,
        pub approver: Option<String>,
        pub cooperate_type: i32,
        pub collaborator: Option<String>,
        pub skip_type: String,
        pub flow_status: String,
        pub form_custom: Option<String>,
        pub form_path: Option<String>,
        pub message: Option<String>,
        pub variable: Option<String>,
        pub ext: Option<String>,
        pub create_time: Option<DateTime>,
        pub update_time: Option<DateTime>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// 流程用户 flow_user
pub mod flow_user {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_user")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        /// 1 审批人 2 转办人 3 委托人
        #[sea_orm(column_name = "type")]
        pub r#type: String,
        pub processed_by: Option<String>,
        pub associated: i64,
        pub create_time: Option<DateTime>,
        pub create_by: Option<String>,
        pub update_time: Option<DateTime>,
        pub update_by: Option<String>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// 流程表单 flow_form
pub mod flow_form {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "flow_form")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub form_code: String,
        pub form_name: String,
        pub version: String,
        pub is_publish: i32,
        pub form_type: Option<i32>,
        pub form_path: Option<String>,
        pub form_content: Option<String>,
        pub ext: Option<String>,
        pub create_time: Option<DateTime>,
        pub update_time: Option<DateTime>,
        pub del_flag: Option<String>,
        pub tenant_id: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
