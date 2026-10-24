use crate::modules::system::domain::SysOperLog;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysOperLogVO {
    pub oper_id: Option<String>,
    pub title: Option<String>,
    pub business_type: Option<i32>,
    pub method: Option<String>,
    pub request_method: Option<String>,
    pub operator_type: Option<i32>,
    pub oper_name: Option<String>,
    pub dept_name: Option<String>,
    pub oper_url: Option<String>,
    pub oper_ip: Option<String>,
    pub oper_location: Option<String>,
    pub oper_param: Option<String>,
    pub json_result: Option<String>,
    pub status: Option<i32>,
    pub error_msg: Option<String>,
    pub oper_time: Option<chrono::NaiveDateTime>,
}

impl From<SysOperLog> for SysOperLogVO {
    fn from(arg: SysOperLog) -> Self {
        Self {
            oper_id: Some(arg.oper_id.to_string()),
            title: arg.title,
            business_type: arg.business_type,
            method: arg.method,
            request_method: arg.request_method,
            operator_type: arg.operator_type,
            oper_name: arg.oper_name,
            dept_name: arg.dept_name,
            oper_url: arg.oper_url,
            oper_ip: arg.oper_ip,
            oper_location: arg.oper_location,
            oper_param: arg.oper_param,
            json_result: arg.json_result,
            status: arg.status,
            error_msg: arg.error_msg,
            oper_time: arg.oper_time,
        }
    }
}
