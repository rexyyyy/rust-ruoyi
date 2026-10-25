//! 工作流引擎的进程内单例。
//!
//! 引擎内部只有 `Arc<dyn WarmStore>` 与若干处理器，克隆只是指针拷贝，
//! 因此用 `OnceCell<Arc<FlowEngine>>` 保存即可，读取端拿 `Arc` 复制一份。
//!
//! 之所以不放进 `AppState`：`AppState` 是「数据库 + redis + 配置」三件套，
//! 由 `db::init_db` 在更早的阶段构造，工作流引擎依赖前者完成后的连接，
//! 单独放一个 OnceCell 可以避免把初始化顺序耦合进 db 模块。

use std::sync::Arc;

use once_cell::sync::OnceCell;
use warm_flow_core::engine::FlowEngine;

use crate::error::{Error, Result};

static ENGINE: OnceCell<Arc<FlowEngine>> = OnceCell::new();

/// 写入全局引擎。重复调用返回 Err（与 OnceCell 语义一致）。
pub fn init(engine: FlowEngine) -> std::result::Result<(), String> {
    ENGINE
        .set(Arc::new(engine))
        .map_err(|_| "workflow engine already initialized".to_string())
}

/// 取引擎（可能尚未初始化）。
pub fn engine() -> Option<&'static Arc<FlowEngine>> {
    ENGINE.get()
}

/// 取引擎，未初始化时转成业务错误而不是 panic。
///
/// 工作流接口全部经由它获取引擎：启动期初始化失败时接口返回「引擎未就绪」，
/// 而不是让整个进程因 unwrap panic 掉。
pub fn engine_checked() -> Result<&'static Arc<FlowEngine>> {
    ENGINE
        .get()
        .ok_or_else(|| Error::from("工作流引擎未初始化，请检查启动日志中 workflow 相关告警"))
}

/// 引擎是否就绪。
pub fn ready() -> bool {
    ENGINE.get().is_some()
}
