use std::time::Duration;

use actix_web::{HttpResponse, Responder, web};
use futures_util::stream;

use crate::modules::system::domain::vo::RespVO;

/// SSE 连接（对应 Spring `${sse.path}` = /resource/sse）
/// 返回 text/event-stream，每 15 秒发送心跳注释保持连接
pub async fn sse_connect() -> impl Responder {
    let stream = stream::unfold(0u32, |mut n| async move {
        tokio::time::sleep(Duration::from_secs(15)).await;
        n += 1;
        Some((Ok::<_, std::io::Error>(web::Bytes::from(": ping\n\n")), n))
    });
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(stream)
}

/// SSE 关闭（对应 Spring `${sse.path}/close` = /resource/sse/close）
pub async fn sse_close() -> impl Responder {
    RespVO::<String>::from_success_info("关闭成功").resp_json()
}
