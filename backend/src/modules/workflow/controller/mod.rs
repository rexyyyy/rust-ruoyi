//! 工作流接口层，挂在 `/workflow/**`。
//!
//! 与 Java 版 `ruoyi-workflow` 的 Controller 一一对应：
//! - [`category`]   `/workflow/category/**`
//! - [`definition`] `/workflow/definition/**`
//! - [`instance`]   `/workflow/instance/**`
//! - [`task`]       `/workflow/task/**`
//! - [`leave`]      `/workflow/leave/**`
//!
//! 设计器自身的接口（保存 JSON、查询流程图、办理人选择）由 `warm-flow-web`
//! 提供，挂在 `/warm-flow/**` 与 `/warm-flow-ui/**`，见 `http_server.rs`。

pub mod category;
pub mod definition;
pub mod instance;
pub mod leave;
pub mod task;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.configure(category::init)
        .configure(definition::init)
        .configure(instance::init)
        .configure(task::init)
        .configure(leave::init);
}
