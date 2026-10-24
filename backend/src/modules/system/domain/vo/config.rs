use crate::modules::system::domain::{SysConfig};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysConfigVO {
    pub config_id: Option<String>,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<String>,
    pub remark: Option<String>,
}

impl From<SysConfig> for SysConfigVO {
    fn from(arg: SysConfig) -> Self {
        Self {
            config_id: Some(arg.config_id.to_string()),
            config_name: arg.config_name,
            config_key: arg.config_key,
            config_value: arg.config_value,
            config_type: arg.config_type,
            remark: arg.remark,
        }
    }
}
