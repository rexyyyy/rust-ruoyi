use crate::modules::system::domain::SysDictType;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysDictTypeVO {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub remark: Option<String>,
}

impl From<SysDictType> for SysDictTypeVO {
    fn from(arg: SysDictType) -> Self {
        Self {
            dict_id: Some(arg.dict_id.to_string()),
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            create_by: arg.create_by.map(|v| v.to_string()),
            create_time: arg.create_time,
            update_by: arg.update_by.map(|v| v.to_string()),
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}

impl SysDictTypeVO {}
