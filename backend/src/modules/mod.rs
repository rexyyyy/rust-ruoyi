//! 业务模块总装（对标 RuoYi Java Spring Boot 的模块化结构）。
//!
//! 分层约定（自上而下，越靠前越底层）：
//!
//! 1. `system` —— 框架核心包（framework-level）。承载用户/角色/菜单/字典/部门/岗位/
//!    参数/通知/字典等最基础的后台管理能力，以及 dao/domain/mapper 等被其它模块共用的
//!    基础设施。相当于 RuoYi 的 `ruoyi-system` + 部分 `ruoyi-framework` 职责。
//!    其它业务模块默认都依赖它，因此它排在最前。
//!
//! 2. `demo` —— **系统框架之外的第一个业务模块（首个外部业务模块 / 模板）**。
//!    用来做新业务模块的样板：完整的 `controller / service / domain` 三件套结构，
//!    后续新增业务模块（如 job、tool 以及任何业务域）都照此骨架搭建。
//!    相当于 RuoYi 的 `ruoyi-demo`。
//!
//! 3. `job` —— 业务模块：动态定时任务（对标 RuoYi `ruoyi-quartz` / `sys_job`）。
//!    基于自研方言无关轮询调度器，提供增删改 / 暂停恢复 / 立即执行 / 执行日志。
//!
//! 4. `tool` —— 业务模块：代码生成器（对标 RuoYi `ruoyi-generator`）。
//!
//! 5. `workflow` —— 业务模块：工作流（对标 RuoYi `ruoyi-workflow`）。
//!    把 `rust-warm-flow` 引擎内嵌进本进程，提供流程分类/定义/实例/任务接口，
//!    以及 warm-flow 设计器所需的扩展点（分类树、办理人选择）。
//!
//! 统一内部约定（每个模块根 `mod.rs` 都显式声明）：
//! - `pub mod controller;`  —— HTTP 接口层（在 `http_server.rs` 中以 `web::scope` 挂载）
//! - `pub mod service;`     —— 业务逻辑层
//! - `pub mod domain;`      —— 实体 / DTO / VO / 分页（sea-orm 实体与请求响应对象）
//! - 共享基础设施（dao/mapper）仅在 `system` 提供，其它模块复用 `system::dao`。
//!
//! 根级核心（`config` / `state` / `error` / `casbin` / `middleware` / `util` 等）留在
//! `src/` 顶层，作为跨模块的公共框架能力，不划入上述业务模块包。
pub mod system; // 框架核心包
pub mod demo;   // 首个业务模块（模板）
pub mod job;    // 业务模块：动态定时任务
pub mod tool;   // 业务模块：代码生成器
pub mod workflow; // 业务模块：工作流（内嵌 warm-flow 引擎）
