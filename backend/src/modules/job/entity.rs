//! 定时任务实体（对标 RuoYi sys_job / sys_job_log）。
//!
//! 与 `crate::modules::system::domain::tables` 的实体约定保持一致：
//! `Model`(数据行) / `Entity` / `ActiveModel` / `Column` 全部由 `DeriveEntityModel`
//! 派生，并 `pub use` 为 `SysXxx` / `SysXxxEntity` 等别名。
//! 为对齐 RuoYi 前端 camelCase 字段，Model 的 serde 命名为 `camelCase`
//! （sea-orm 列映射走 `#[sea_orm(...)]`，不受 serde 影响）。

// ===================== sys_job =====================
pub mod sys_job {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_job")]
    #[serde(rename_all = "camelCase")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub job_id: i64,
        pub job_name: Option<String>,
        pub job_group: Option<String>,
        #[serde(rename = "invokeTarget")]
        pub invoke_target: Option<String>,
        #[serde(rename = "cronExpression")]
        pub cron_expression: Option<String>,
        #[serde(rename = "misfirePolicy")]
        pub misfire_policy: Option<String>,
        pub concurrent: Option<String>,
        pub status: Option<String>,
        #[serde(rename = "createBy")]
        pub create_by: Option<String>,
        #[serde(rename = "createTime")]
        pub create_time: Option<NaiveDateTime>,
        #[serde(rename = "updateBy")]
        pub update_by: Option<String>,
        #[serde(rename = "updateTime")]
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_job::Model as SysJob;
pub use sys_job::Entity as SysJobEntity;
pub use sys_job::ActiveModel as SysJobActiveModel;
pub use sys_job::Column as SysJobColumn;

// ===================== sys_job_log =====================
pub mod sys_job_log {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_job_log")]
    #[serde(rename_all = "camelCase")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub job_log_id: i64,
        pub job_name: Option<String>,
        pub job_group: Option<String>,
        #[serde(rename = "invokeTarget")]
        pub invoke_target: Option<String>,
        #[serde(rename = "jobMessage")]
        pub job_message: Option<String>,
        pub status: Option<String>,
        #[serde(rename = "exceptionInfo")]
        pub exception_info: Option<String>,
        #[serde(rename = "createTime")]
        pub create_time: Option<NaiveDateTime>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_job_log::Model as SysJobLog;
pub use sys_job_log::Entity as SysJobLogEntity;
pub use sys_job_log::ActiveModel as SysJobLogActiveModel;
pub use sys_job_log::Column as SysJobLogColumn;
