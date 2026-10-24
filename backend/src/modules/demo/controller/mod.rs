//! DemoCourse 路由注册（codegen 生成）
//!
//! actix-web 按注册顺序匹配路由，所以固定路径（list / export）必须排在
//! 带参数的动态路径（`/{id}`）之前，否则动态路径会把它们吞掉。

use actix_web::web;

pub mod course_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(course_controller::page)
        .service(course_controller::export)
        .service(course_controller::detail)
        .service(course_controller::add)
        .service(course_controller::update)
        .service(course_controller::remove);
}
