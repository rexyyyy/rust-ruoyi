//! 工作流模块：把 `rust-warm-flow` 引擎嵌入 RuoYi-Rust。
//!
//! 对标 Java 版 `ruoyi-modules/ruoyi-workflow`（Dromara RuoYi-Vue-Plus）：
//! warm-flow 只负责「流程怎么流转」，业务侧（本模块）负责「谁能办、办完业务表怎么改」。
//!
//! 分层：
//! - [`state`]      —— 进程内 `FlowEngine` 单例（引擎持有 `Arc<dyn WarmStore>`）
//! - [`permission`] —— 把设计器里的 `role:1` / `dept:101` 翻译成真实用户 id
//! - [`ui`]         —— 设计器扩展点（分类树 / 办理人选择 / 表单路径）
//! - [`domain`]     —— 业务表实体（flow_category / test_leave）与 DTO/VO
//! - [`service`]    —— 分类、定义、实例、任务、请假五块业务
//! - [`controller`] —— actix-web 接口，挂在 `/workflow/**`
//!
//! 设计器自身的接口（保存 JSON、查询流程图等）由 `warm-flow-web` 直接提供，
//! 挂在 `/warm-flow/**`，与本模块共用同一个引擎实例。

pub mod controller;
pub mod domain;
pub mod permission;
pub mod service;
pub mod state;
pub mod ui;

use std::sync::Arc;

use warm_flow_core::engine::FlowEngine;
use warm_flow_core::handler::WarmFlowConfig;
use warm_flow_orm::SeaOrmStore;

use self::permission::RuoYiPermissionHandler;

/// 初始化工作流引擎。由 `main` 在数据库就绪后调用一次。
///
/// 失败只影响工作流功能，不阻断整个后台启动（返回 Err 由调用方决定是否忽略）。
pub async fn init() -> Result<(), String> {
    let db = crate::db::db()
        .ok_or_else(|| "数据库未初始化，无法启动工作流引擎".to_string())?
        .clone();

    let store = Arc::new(SeaOrmStore::new(db.clone()));
    let permission = Arc::new(RuoYiPermissionHandler::new());
    // 首次全量加载角色/部门 → 用户映射；失败不阻断（办理人展开时按空处理，可重试刷新）
    if let Err(e) = permission.refresh().await {
        log::warn!("[workflow] 办理人缓存首次加载失败: {}", e);
    }

    let config = WarmFlowConfig {
        // 设计器接口由 warm-flow-web 提供，这里保留开关语义
        ui: true,
        // 引擎内部 id 生成器：19 位雪花，与前端字符串主键配套
        key_type: Some("SnowId19".to_string()),
        // 与 RuoYi 前端保持一致的鉴权头
        token_name: "Authorization".to_string(),
        // 流程图顶部文字（实例名称/状态）
        top_text_show: true,
        banner: false,
        ..Default::default()
    };

    let engine = FlowEngine::new(store)
        .with_config(config)
        .with_permission(permission.clone());

    state::init(engine)?;
    // 角色/部门变更后最多滞后 30s 生效；办理人配置不会频繁变动，够用且零锁竞争
    permission.spawn_refresher(std::time::Duration::from_secs(30));

    // 设计器扩展点：分类树快照（同步接口，数据在库里）
    ui::init().await;

    // 内置示例流程：库里没发布过 leave1 时自动导入，保证开箱可跑
    if let Err(e) = service::demo::ensure_demo_flow().await {
        log::warn!("[workflow] 示例流程初始化失败: {}", e);
    }

    log::info!("[workflow] warm-flow 引擎已启动（RuoYi 权限处理器 + SeaORM 存储）");
    Ok(())
}
