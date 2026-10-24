//! RuoYi-Rust 代码生成引擎。
//!
//! 职责边界：
//! - **本 crate**：读元数据 -> 算上下文 -> 渲染模板 -> 产出文件或 zip；
//! - **backend 的 `modules::tool` 模块**：负责 `gen_table` / `gen_table_column`
//!   的持久化与 HTTP 接口，调用这里的能力。
//!
//! 因此这里刻意不引入任何 actix-web 依赖，也不感知 gen_table 表结构，
//! 便于单独用命令行跑（`codegen-cli`）或在测试里直接调用。

pub mod db;
pub mod error;
pub mod gen;
pub mod meta;
pub mod naming;
pub mod service;
pub mod template;
pub mod util;

pub use error::{Error, Result};
pub use gen::context::{ColCtx, GenColumnOverride, GenCtx, GenOptions, PkCtx};
pub use gen::GenFile;
pub use meta::{ColumnMeta, DbTableRow, TableMeta};
pub use service::CodegenEngine;
