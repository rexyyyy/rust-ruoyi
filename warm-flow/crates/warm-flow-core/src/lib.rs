//! Warm-Flow 工作流引擎内核。
//!
//! 对应 Java 版 `org.dromara.warm.flow.core`，按职责拆分为：
//! - [`entity`]：7 张流程表 + 1 张表单表的领域对象；
//! - [`enums`]：节点类型、跳转类型、流程状态等枚举；
//! - [`dto`]：设计器交互用的 JSON 结构与流程入参 `FlowParams`；
//! - [`store`]：持久层抽象，`warm-flow-orm` 用 SeaORM 实现；
//! - [`engine`]：流程引擎（开启、审批、退回、撤销、终止、转办、委派、加减签、拿回等）。
//!
//! 与原实现的差异：Java 通过 `FlowEngine` 静态字段 + Spring 容器取 bean，
//! Rust 版改为 `FlowEngine` 实例持有 `Arc<dyn WarmStore>` 与各类处理器，
//! 便于多数据源与单元测试隔离。

pub mod chart;
pub mod constant;
pub mod dto;
pub mod engine;
pub mod entity;
pub mod enums;
pub mod error;
pub mod expression;
pub mod handler;
pub mod store;
pub mod utils;

pub use error::{FlowError, Result};
