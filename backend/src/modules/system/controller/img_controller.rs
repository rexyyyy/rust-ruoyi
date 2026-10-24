use std::time::Duration;

use actix_web::Responder;
use captcha::Captcha;
//use captcha::filters::{Dots, Noise, Wave};
use captcha::filters::{ Noise, Wave};
use uuid::Uuid;

use crate::config::cache_variables::REDIS_UUID_CAPTCHA;
use crate::modules::system::domain::vo::{RespJson, RespVO};
use crate::modules::system::service::CONTEXT;
use crate::util::base64::encode;

pub async fn captcha() -> impl Responder {
    // 前端拦截器返回整个 body，login.vue 解构 body.data 取验证码字段，必须包 data 层（对齐 Java 版）
    let mut json = RespJson::success();

    if CONTEXT.config.captcha_enabled {
        let id = Uuid::new_v4();
        let mut captcha = Captcha::new();
        captcha
            .set_chars(&CONTEXT.char_array)
            .add_chars(4)
            .set_color([20,20,250])
            .apply_filter(Noise::new(0.1))
            .apply_filter(Wave::new(1.0, 10.0).horizontal())
            // .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(160, 60)
            // .apply_filter(Dots::new(4))
            ;
        let png = captcha.as_png().unwrap();
        let captcha_str = captcha.chars_as_string().to_lowercase();
        // 不再打印明文验证码到日志（即使 debug 模式也避免敏感信息落盘）

        let result = CONTEXT
            .cache_service
            .set_string_ex(
                &format!("{}{}", REDIS_UUID_CAPTCHA, &id.to_string()),
                captcha_str.as_str(),
                Some(Duration::from_secs(
                    CONTEXT.config.captcha_expired_min * 60
                )),
            )
            .await;
        //println!("{:?}", result);
        if CONTEXT.config.debug == false {
            //release mode, return the error
            if result.is_err() {
                return RespVO::from_result(&result).resp_json();
            }
        }
        let mut data = serde_json::Map::new();
        data.insert("captchaEnabled".to_string(), true.into());
        data.insert("uuid".to_string(), id.to_string().into());
        data.insert("img".to_string(), encode(&png).into());
        // plus-ui 5.x login.vue: const { data } = res 后取 data.img/data.uuid，
        // 对齐 RuoYi-Vue-Plus Java 版 R<CaptchaVo> —— 业务字段必须包在 data 层
        json.insert("data".to_string(), serde_json::Value::Object(data));
    } else {
        let mut data = serde_json::Map::new();
        data.insert("captchaEnabled".to_string(), false.into());
        json.insert("data".to_string(), serde_json::Value::Object(data));
    }
    json.resp_json()
}
