//! DemoCourse sea-orm 实体
//! 源表：demo_course（演示课程表）
//!
//! 由 codegen 于 2026-10-08 21:03:18 生成，重新生成时本文件会被覆盖。
//!
//! 说明：非原生类型统一写成全限定路径（`chrono::NaiveDateTime` 等），
//! 因此除 prelude 外不需要任何额外 `use`，也就不存在多余的未使用导入。

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "demo_course")]
pub struct Model {
    // 主键在 DTO 层显式赋值（雪花 ID / uuid），故一律 auto_increment = false
    #[sea_orm(primary_key, auto_increment = false)]
    pub course_id: i64,
    pub course_code: Option<String>, // 课程编码
    pub course_name: Option<String>, // 课程名称
    pub teacher: Option<String>, // 授课教师
    pub credit: Option<f64>, // 学分
    pub open_status: Option<String>, // 开课状态
    pub start_date: Option<chrono::NaiveDate>, // 开课日期
    pub remark: Option<String>, // 备注
    pub create_by: Option<i64>, // 创建者
    pub create_time: Option<chrono::NaiveDateTime>, // 创建时间
    pub update_by: Option<i64>, // 更新者
    pub update_time: Option<chrono::NaiveDateTime>, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// 别名沿用 backend 既有模块（sys_post 等）的书写习惯
pub use Model as DemoCourse;
pub use Entity as DemoCourseEntity;
pub use ActiveModel as DemoCourseActiveModel;
pub use Column as DemoCourseColumn;
