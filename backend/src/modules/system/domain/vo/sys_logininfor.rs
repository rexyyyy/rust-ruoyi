use crate::modules::system::domain::{SysLogininfor};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysLogininforVO {
    pub info_id: Option<String>,
    pub user_name: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub status: Option<String>,
    pub msg: Option<String>,
    pub login_time: Option<chrono::NaiveDateTime>,
}

impl From<SysLogininfor> for SysLogininforVO {
    fn from(arg: SysLogininfor) -> Self {
        Self {
            info_id: Some(arg.info_id.to_string()),
            user_name: arg.user_name,
            ipaddr: arg.ipaddr,
            login_location: arg.login_location,
            browser: arg.browser,
            os: arg.os,
            status: arg.status,
            msg: arg.msg,
            login_time: arg.login_time
        }
    }
}
