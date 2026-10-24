use crate::modules::system::domain::{SysDictData};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysDictDataVO {

    pub dict_code: Option<String>,
    pub dict_sort: Option<i32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_type: Option<String>,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub remark: Option<String>,
}

impl From<SysDictData> for SysDictDataVO {
    fn from(arg: SysDictData) -> Self {
        Self {
            dict_code: Some(arg.dict_code.to_string()),
            dict_sort: arg.dict_sort,
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            dict_type: arg.dict_type,
            css_class: arg.css_class,
            list_class: arg.list_class,
            is_default: arg.is_default,
            create_by: arg.create_by.map(|v| v.to_string()),
            create_time: arg.create_time,
            update_by: arg.update_by.map(|v| v.to_string()),
            update_time: arg.update_time,
            remark: arg.remark,
        }
    }
}

impl SysDictDataVO {}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysDictDataSimpleVO {
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
}

impl From<SysDictData> for SysDictDataSimpleVO {
    fn from(arg: SysDictData) -> Self {
        Self {
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            css_class: arg.css_class,
            list_class: arg.list_class,
        }
    }
}

impl SysDictDataSimpleVO {}
