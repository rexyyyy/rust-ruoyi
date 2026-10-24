use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::modules::system::domain::PageRequest;


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LogininforPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    /// 登录地址（模糊匹配）
    #[serde(default)]
    pub ipaddr: Option<String>,
    /// 用户名称（模糊匹配）；前端字段名为 camelCase
    #[serde(rename(deserialize = "userName"), alias = "user_name", default)]
    pub user_name: Option<String>,
    /// 登录状态 0成功 1失败
    #[serde(default)]
    pub status: Option<String>,
    /// RuoYi 约定的日期范围参数：`params[beginTime]` / `params[endTime]`
    #[serde(default)]
    pub params: Option<HashMap<String, String>>,
}

impl From<LogininforPageDTO> for PageRequest {
    fn from(arg: LogininforPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&LogininforPageDTO> for PageRequest {
    fn from(arg: &LogininforPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
