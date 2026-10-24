use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub username: String,
    pub password: String,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    pub code: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignUpDTO {
    pub username: Option<String>,
    pub password: Option<String>,
    pub code: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SocialCallbackDTO {
    pub source: Option<String>,
    pub code: Option<String>,
    pub state: Option<String>,
}