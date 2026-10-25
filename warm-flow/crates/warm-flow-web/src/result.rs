//! 统一响应封装。
//!
//! Java 版所有接口都返回 `ApiResult<T>`（`{code, msg, data}`），HTTP 状态码基本恒为 200，
//! 业务失败通过 `code = 500` 表达。这里保持一致：`FlowError::Biz` / `NotFound` 返回
//! HTTP 200 + `code = 500`，只有底层数据库异常才落到 HTTP 500，便于日志定位。

use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use warm_flow_core::error::{ApiResult, FlowError};

/// 接口返回值包装：`WebResult(engine.pass(id, params).await)`
pub struct WebResult<T>(pub Result<T, FlowError>);

impl<T> WebResult<T> {
    pub fn ok(value: T) -> Self {
        Self(Ok(value))
    }
}

impl<T> From<FlowError> for WebResult<T> {
    fn from(err: FlowError) -> Self {
        Self(Err(err))
    }
}

impl<T: Serialize> Responder for WebResult<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self.0 {
            Ok(data) => HttpResponse::Ok().json(ApiResult::ok(data)),
            Err(err) => {
                let (status, code, msg) = error_payload(&err);
                if status >= 500 {
                    log::error!("warm-flow 接口异常: {err}");
                } else {
                    log::warn!("warm-flow 业务校验未通过: {err}");
                }
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap_or(
                    actix_web::http::StatusCode::OK,
                ))
                .json(ApiResult::<()>::fail_with(code, msg))
            }
        }
    }
}

/// 无返回数据的成功响应，等价于 Java `ApiResult.ok()`
pub struct WebDone;

impl Responder for WebDone {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(ApiResult::<()>::ok_empty())
    }
}

fn error_payload(err: &FlowError) -> (u16, i32, String) {
    match err {
        // 业务校验失败：前端直接弹 msg
        FlowError::Biz(msg) => (200, 500, msg.clone()),
        FlowError::NotFound(msg) => (200, 500, msg.clone()),
        FlowError::Db(_) => (500, 500, err.to_string()),
    }
}
