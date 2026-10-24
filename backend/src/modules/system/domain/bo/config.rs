use crate::modules::system::domain::SysConfig;
use crate::modules::system::domain::PageRequest;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_type: Option<String>,
    pub status: Option<String>,

}

impl From<ConfigPageDTO> for PageRequest {
    fn from(arg: ConfigPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&ConfigPageDTO> for PageRequest {
    fn from(arg: &ConfigPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigAddDTO {
    pub config_id: Option<String>,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<String>,
    pub remark: Option<String>,
}

impl From<ConfigAddDTO> for SysConfig {
    fn from(arg: ConfigAddDTO) -> Self {
        SysConfig {
            config_id: crate::util::idseq::next_id() as i64,
            config_name: arg.config_name,
            config_key: arg.config_key,
            config_value: arg.config_value,
            config_type: arg.config_type,
            create_by: None,
            create_time: chrono::Local::now().naive_local().into(),
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigUpdateDTO {
    pub config_id: Option<String>,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<String>,
    pub remark: Option<String>,
}

impl From<ConfigUpdateDTO> for SysConfig {
    fn from(arg: ConfigUpdateDTO) -> Self {
        SysConfig {
            config_id: arg.config_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
            config_name: arg.config_name,
            config_key: arg.config_key,
            config_value: arg.config_value,
            config_type: arg.config_type,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: chrono::Local::now().naive_local().into(),
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigUpdateByKeyDTO {
    pub config_key: Option<String>,
    pub config_value: Option<String>,
}