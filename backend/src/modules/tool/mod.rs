//! `tool` —— 业务模块：代码生成器（对标 RuoYi `ruoyi-generator`）。
//!
//! - 本模块负责 `gen_table` / `gen_table_column` 的持久化与 `/tool/gen/*` HTTP 接口；
//! - 真正的「读元数据 -> 渲染模板 -> 产出文件/zip」由 `codegen` crate 提供。

pub mod controller;
pub mod domain;
pub mod service;
