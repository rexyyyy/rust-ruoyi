use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::modules::system::domain::PageRequest;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OperLogPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    /// 系统模块（模糊匹配）
    #[serde(default)]
    pub title: Option<String>,
    /// 操作人员（模糊匹配）；前端字段名为 camelCase
    #[serde(rename(deserialize = "operName"), alias = "oper_name", default)]
    pub oper_name: Option<String>,
    /// 业务类型 0其它 1新增 2修改 3删除 …
    #[serde(rename(deserialize = "businessType"), alias = "business_type", default)]
    pub business_type: Option<i32>,
    /// 操作状态 0正常 1异常
    #[serde(default)]
    pub status: Option<i32>,
    /// RuoYi 约定的日期范围参数：`params[beginTime]` / `params[endTime]`
    #[serde(default)]
    pub params: Option<HashMap<String, String>>,
}

impl From<&OperLogPageDTO> for PageRequest {
    fn from(arg: &OperLogPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
