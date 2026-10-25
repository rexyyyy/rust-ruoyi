//! 工作流领域层。
//!
//! - [`entity`] —— 本模块自己的业务表（flow_category / test_leave）；
//! - [`bo`]     —— 接口入参；
//! - [`vo`]     —— 接口出参。
//!
//! warm-flow 自身的表（flow_definition / flow_node / flow_task ...）由
//! `warm-flow-orm` 提供实体，这里不再重复定义，统一通过引擎的 `WarmStore` 访问。

pub mod bo;
pub mod entity;
pub mod vo;

pub use entity::{FlowCategory, FlowCategoryActiveModel, FlowCategoryColumn, FlowCategoryEntity};
pub use entity::{TestLeave, TestLeaveActiveModel, TestLeaveColumn, TestLeaveEntity};

/// 复用系统模块的分页结构，避免各模块重复定义。
pub use crate::modules::system::domain::{Page, PageRequest};
