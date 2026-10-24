use actix_web::{ get, Responder, web};

use crate::modules::system::domain::vo::RespVO;
use crate::util::hardware::get_server_info;

#[get("/server")]
pub async fn server_info() -> impl Responder {
    // sysinfo 的 System::new_all()/refresh_all() 是同步阻塞且较重，
    // 放进 actix 的阻塞线程池执行，避免卡住 tokio worker。
    let info = web::block(get_server_info)
        .await
        .map_err(|e| crate::error::Error::from(e.to_string()));
    RespVO::from_result(&info).resp_json()
}
