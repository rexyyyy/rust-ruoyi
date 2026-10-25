//! Warm-Flow 的 actix-web 接口层。
//!
//! 对应 Java 版两处：
//! - `warm-flow-plugin-ui-sb-web` 的 `WarmFlowController`（`/warm-flow/**`，设计器用）；
//! - 业务系统的流程 CRUD / 审批接口（Java 版由各业务项目自行实现，这里一并给出通用实现）。
//!
//! 挂载方式：
//! ```no_run
//! # use std::sync::Arc;
//! # use actix_web::{App, HttpServer, web};
//! # async fn demo(engine: Arc<warm_flow_core::engine::FlowEngine>) -> std::io::Result<()> {
//! let state = web::Data::new(warm_flow_web::FlowWebState::new(engine));
//! HttpServer::new(move || {
//!     App::new()
//!         .app_data(state.clone())
//!         .configure(warm_flow_web::configure)
//! })
//! .bind(("0.0.0.0", 8080))?
//! .run()
//! .await
//! # }
//! ```

pub mod controller;
pub mod extension;
pub mod query;
pub mod result;
pub mod vo;

use std::sync::Arc;

use actix_web::web;

use warm_flow_core::engine::FlowEngine;

pub use extension::{DefaultUiExtension, UiExtension};
pub use result::{WebDone, WebResult};

/// 接口层共享状态：引擎 + 设计器扩展点
pub struct FlowWebState {
    engine: Arc<FlowEngine>,
    ui: Arc<dyn UiExtension>,
}

impl FlowWebState {
    pub fn new(engine: Arc<FlowEngine>) -> Self {
        Self { engine, ui: Arc::new(DefaultUiExtension) }
    }

    /// 注入业务侧的设计器扩展点
    pub fn with_ui(mut self, ui: Arc<dyn UiExtension>) -> Self {
        self.ui = ui;
        self
    }

    pub fn engine(&self) -> &FlowEngine {
        &self.engine
    }

    pub fn ui(&self) -> &Arc<dyn UiExtension> {
        &self.ui
    }
}

/// 注册全部路由。设计器接口路径与 Java 版逐一对齐。
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/warm-flow")
            .configure(controller::designer::routes)
            .service(web::scope("/definition").configure(controller::definition::routes))
            .service(web::scope("/instance").configure(controller::instance::routes))
            .service(web::scope("/task").configure(controller::task::routes))
            .service(web::scope("/form").configure(controller::form::routes)),
    )
    .service(
        web::scope("/warm-flow-ui").route("/config", web::get().to(controller::designer::config)),
    );
}
