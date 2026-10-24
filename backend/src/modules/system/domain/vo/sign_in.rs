use chrono::NaiveDateTime;
use crate::modules::system::domain::vo::user::SysUserVO;
use serde::{Deserialize, Serialize};
use crate::modules::system::domain::SysRole;

///的后的所有信息，保存到redis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCache {
    pub id: i64,
    pub user_name: String,
    pub user: Option<SysUserVO>,
    pub permissions: Vec<String>,
    pub menu_ids: Vec<i64>,
    pub roles: Vec<SysRole>,
    pub login_time: NaiveDateTime,
    pub token_key:String
}

impl ToString for UserCache {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
