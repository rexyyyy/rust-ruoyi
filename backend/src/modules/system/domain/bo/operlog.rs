use serde::{Deserialize, Serialize};
use crate::modules::system::domain::PageRequest;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OperLogPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub title: Option<String>,
    pub oper_name: Option<String>,
    pub business_type: Option<i32>,
    pub status: Option<i32>,
}

impl From<&OperLogPageDTO> for PageRequest {
    fn from(arg: &OperLogPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
