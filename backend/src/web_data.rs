use actix_web::{FromRequest, HttpRequest};
use actix_web::HttpMessage;
use std::future::{ready, Ready};

/// 当前登录用户（请求作用域）。
///
/// 由 `middleware::auth_actix::Auth` 在鉴权成功后写入 `req.extensions()`，
/// handler 通过形参 `current_user: CurrentUser` 提取。
///
/// 相比原先的 `thread_local! + RefCell` 全局态：
/// - 天然按请求隔离，不会因 Actix 多线程异步运行时跨线程 `await` 而把 A 请求的用户
///   串到 B 请求上（原先的 `create_by/update_by` 可能写成别人的用户，是隐藏的正确性 bug）；
/// - 无全局可变状态、无需锁、无 `unsafe`；
/// - 依赖显式从请求传入，可测试、可读。
#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: i64,
    pub name: String,
}

impl FromRequest for CurrentUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_http::Payload) -> Self::Future {
        match req.extensions().get::<CurrentUser>() {
            Some(u) => ready(Ok(u.clone())),
            // 未注入说明请求未走 Auth 中间件（或白名单接口本就无用户），返回 401
            None => ready(Err(actix_web::error::ErrorUnauthorized("missing current user"))),
        }
    }
}
