//! oms 模块（由 codegen 于 2026-10-10 16:24:42 生成）
//!
//! 接入 backend 只需两步：
//! 1. `src/modules/mod.rs` 里加 `pub mod oms;`
//! 2. `src/http_server.rs` 里加
//!    ```
//!    web::scope("/oms").configure(oms::controller::init)
//!    ```

pub mod controller;
pub mod domain;
pub mod service;
