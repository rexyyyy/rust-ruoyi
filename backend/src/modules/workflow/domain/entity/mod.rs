//! 工作流业务表实体（不属于 warm-flow 内核，由 RuoYi 侧维护）。
//!
//! sea-orm 的 `DeriveEntityModel` 会固定生成 `Entity` / `Column` / `ActiveModel` /
//! `Relation` 四个名字，因此**一个实体一个文件**，再由本模块统一 re-export。

pub mod flow_category;
pub mod test_leave;

pub use flow_category::{
    ActiveModel as FlowCategoryActiveModel, Column as FlowCategoryColumn,
    Entity as FlowCategoryEntity, Model as FlowCategory,
};
pub use test_leave::{
    ActiveModel as TestLeaveActiveModel, Column as TestLeaveColumn, Entity as TestLeaveEntity,
    Model as TestLeave,
};
