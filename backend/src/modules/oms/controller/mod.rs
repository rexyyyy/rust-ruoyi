//! OmsRegorder 路由注册（codegen 生成）
//!
//! actix-web 按注册顺序匹配路由，所以固定路径（list / export）必须排在
//! 带参数的动态路径（`/{id}`）之前，否则动态路径会把它们吞掉。

use actix_web::web;

pub mod regorder_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(regorder_controller::page)
        .service(regorder_controller::export)
        .service(regorder_controller::detail)
        .service(regorder_controller::add)
        .service(regorder_controller::update)
        .service(regorder_controller::remove);
}
