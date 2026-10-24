//use actix::{Actor, StreamHandler};
use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, Session};
// use actix_ws::{Message, Session};
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
// 用 tokio 异步锁：原 std::sync::Mutex 会在持锁期间 .await（发送 ws 消息），
// 既阻塞执行器又有死锁风险；tokio 锁 + 「先取会话、锁外发送」避免跨 await 持锁。
use tokio::sync::Mutex;
// 消息数据结构
#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    content: String,
    message_type: String,
    recipient_id: Option<i32>,
}

// 连接信息结构
pub struct Connection {
    session: Session,
}
static SREG: Lazy<Arc<Mutex<HashMap<i32, Connection>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// #[get("/ws/{user_id}")]
pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let (response, mut session, stream) = actix_ws::handle(&req, stream)?;
    // 如果用户没有断开连接的情况下多次连接，先删除之前的连接，始终保持一个用户一个最新连接
    // 先在锁内取出旧 session 的克隆，锁外再 close，避免持锁 .await
    let old_session = { SREG.lock().await.get(&user_id).map(|c| c.session.clone()) };
    if let Some(old) = old_session {
        // 关闭旧连接
        let _ = old.close(None).await;
        SREG.lock().await.remove(&user_id);

        println!("关闭旧连接: {}", &user_id);
    }
    println!("user in : {}", &user_id);
    let mut a_stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    SREG.lock().await.insert(
        user_id,
        Connection {
            session: session.clone(),
        },
    );

    //  (user_id, Connection { session: session.clone() });
    // respond immediately with response connected to WS session

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = a_stream.next().await {
            match msg {
                Ok(AggregatedMessage::Close(close_reason)) => {
                    // echo binary message
                    SREG.lock().await.remove(&user_id);
                    println!("连接 {} 已关闭", user_id);
                }
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    //session.text(text).await.unwrap();
                    println!("websocket text: {}", text);
                    handle_text_message(&text, user_id).await;
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    // session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    Ok(response)
}

async fn handle_text_message(text: &str, user_id: i32) {
    // 解析消息对象
    let msg = match serde_json::from_str::<ChatMessage>(text) {
        Ok(m) => m,
        Err(e) => {
            println!("ws消息解析失败: {}", e);
            return;
        }
    };
    // 根据消息类型处理
    match msg.message_type.as_str() {
        "private" => send_private_message(&msg).await, // 将消息发给 recipient_id
        "broadcast" => broadcast_message(&msg.content).await, // 将消息广播给所有连接
        "heartbeat" => heartbeat_message(&user_id).await, // 心跳消息
        _ => eprintln!("未知消息类型: {}", msg.message_type),
    }
}
// 公用方法发送消息
pub async fn send_message(user_id: i32, message: &str) {
    // 锁内仅收集目标 session 克隆，锁外发送
    let sessions: Vec<Session> = {
        let reg_conns = SREG.lock().await;
        reg_conns
            .iter()
            .filter(|(k, _)| **k == user_id)
            .map(|(_, c)| c.session.clone())
            .collect()
    };

    for mut conn in sessions {
        let content = ChatMessage {
            content: message.to_string(),
            message_type: "reply".to_string(), // 服务器回复消息
            recipient_id: None,
        };
        let json_content = serde_json::to_string(&content).unwrap();

        if let Err(e) = conn.text(json_content).await {
            eprintln!("发送失败:to {} err {}", user_id, e);
        }
    }

    // let mut reg_guard = SREG.lock().unwrap();
    // let conn = reg_guard.get_mut(&user_id);
    // match conn {
    //     Some(c) => {
    //         c.session.text(content).await.unwrap();
    //     }
    //     None => {
    //         eprintln!("初始化连接失败: {}", user_id);
    //         return;
    //     }
    // }
}
// 心跳
async fn heartbeat_message(user_id: &i32) {
    // 处理逻辑

    // let mut failed_conns = Vec::new();

    // let reg = SREG.lock().unwrap();

    // for key in reg.keys() {
    //     if key == user_id {
    //         // 发送心跳消息
    //         let mut reg_guard = SREG.lock().unwrap();
    //         let conn = reg_guard.get_mut(key);
    //         match conn {
    //             Some(c) => {
    //                 c.session
    //                     .text("{\"message_type\":\"heartbeat\"}")
    //                     .await
    //                     .unwrap();
    //             }
    //             None => {
    //                 eprintln!("心跳发送失败: {}", user_id);
    //                 failed_conns.push(*key);
    //                 return;
    //             }
    //         }
    //         // reg_guard is dropped here
    //     }
    // }

    // for conn_id in failed_conns {
    //     SREG.lock().unwrap().remove(&conn_id);
    // }
}

// 私聊消息发送
async fn send_private_message(msg: &ChatMessage) {
    let to_userid = msg.recipient_id.unwrap_or(-1);
    // 接收到消息以后处理
    if to_userid < 0 {
        eprintln!("发送ws消息失败,没有接收者: {}", to_userid);
        return;
    }

    // let mut failed_conns = Vec::new();

    // let reg = SREG.lock().unwrap();
    // // 找到接收者
    // let mut reg_guard = SREG.lock().unwrap();
    // let conn = reg_guard.get_mut(&to_userid);

    // match conn {
    //     Some(c) => {
    //         eprintln!("发送ws消息给: {}", to_userid);
    //         if let Err(e) = c.session.text(msg.content.clone()).await {
    //             eprintln!("发送ws消息失败,连接无效: {}", e);
    //             failed_conns.push(to_userid);
    //         }
    //     }
    //     None => {
    //         eprintln!("发送ws消息失败,没有连接: {}", to_userid);

    //         return;
    //     }
    // }

    // // 清理失效连接
    // for conn_id in failed_conns {
    //     SREG.lock().unwrap().remove(&conn_id);
    // }
}

// 广播消息
async fn broadcast_message(message: &str) {
    // 锁内仅收集全部 session 克隆，锁外发送
    let sessions: Vec<Session> = {
        let reg_conns = SREG.lock().await;
        reg_conns.values().map(|c| c.session.clone()).collect()
    };

    for mut conn in sessions {
        let content = ChatMessage {
            content: message.to_string(),
            message_type: "reply".to_string(), // 服务器回复消息
            recipient_id: None,
        };
        let json_content = serde_json::to_string(&content).unwrap();
        if let Err(e) = conn.text(json_content).await {
            eprintln!("广播失败: {}", e);
        }
    }
}

// // #[get("/ws")]
// pub async fn websocket_handler2(req: HttpRequest, stream: web::Payload) -> impl Responder {
//     dbg!("user in");
//     println!("user in");
//     let resp = actix_web_actors::ws::start(MyWebSocket, &req, stream);
//     match resp {
//         Ok(res) => res,
//         Err(e) => HttpResponse::InternalServerError().body(format!("WebSocket握手失败: {}", e)),
//     }
// }

// struct MyWebSocket;

// impl Actor for MyWebSocket {
//     type Context = actix_web_actors::ws::WebsocketContext<Self>;
// }

// impl StreamHandler<Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>>
//     for MyWebSocket
// {
//     fn handle(
//         &mut self,
//         msg: Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>,
//         ctx: &mut Self::Context,
//     ) {
//         match msg {
//             Ok(actix_web_actors::ws::Message::Text(text)) => {
//                 // 处理接收到的文本消息
//                 ctx.text(text);
//             }
//             Ok(actix_web_actors::ws::Message::Close(reason)) => {
//                 // 处理连接关闭
//                 ctx.close(reason);
//                 //ctx.
//             }
//             _ => (),
//         }
//     }
// }
