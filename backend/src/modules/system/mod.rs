//! `system` —— 框架核心包（framework-level module，对标 RuoYi `ruoyi-system`）。
//!
//! 承载后台最基础的管理能力（用户/角色/菜单/字典/部门/岗位/参数/通知等），
//! 并提供被其它业务模块共用的基础设施：`dao`（通用 CRUD 助手）、
//! `domain`（sea-orm 实体 / 分页 / BO / VO）、`mapper`。
//!
//! 在 `src/modules/mod.rs` 中排在最前，其它业务模块（`demo`/`job`/`tool`）默认依赖它。
pub mod controller;
pub mod dao;
pub mod domain;
pub mod mapper;
pub mod service;
