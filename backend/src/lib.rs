#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]
extern crate log;
extern crate crypto;//rust-crypto
// extern crate bcrypt;

#[macro_use]
pub mod util;
pub mod config;
// pub mod  controller;
// pub mod domain;
pub mod error;
// pub mod service;
pub mod modules;
pub mod web_data;
pub mod http_server;
// 全局 sea-orm 连接（orm 迁移阶段基础）
pub mod db;
// 应用全局状态：DatabaseConnection + redis 客户端 + ApplicationConfig，
// 通过 web::Data<AppState> 注入 actix app state（对齐 sea-orm 官方示例）
pub mod state;
// 基于 casbin 的 RBAC-with-domains 权限层
pub mod casbin;
//管理token，为兼容若依管理平台重新写，不再使用中间件进行验证
pub mod token_auth;
// 统一鉴权中间件（auth_actix）。auth.rs 是旧路径实现（crate::service / crate::domain
// 已在 orm 迁移后不存在），保留代码但不编译。
pub mod middleware;

/// 启动栈溢出排查用：直接写文件的打点。
///
/// - **仅 debug 构建生效**：release 下为空操作，避免生产环境每次打点都开一次文件
///   （字典/菜单刷新等请求路径上也会调用）。
/// - 日志路径由环境变量 `RUOYI_CKPT_FILE` 指定，未设置时不写文件，只走 `log::debug!`。
pub fn ckpt(msg: &str) {
    #[cfg(debug_assertions)]
    {
        match std::env::var("RUOYI_CKPT_FILE") {
            Ok(path) if !path.trim().is_empty() => {
                use std::io::Write;
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
                    let _ = writeln!(f, "{}", msg);
                }
            }
            _ => log::debug!("[ckpt] {}", msg),
        }
    }
    #[cfg(not(debug_assertions))]
    {
        let _ = msg;
    }
}