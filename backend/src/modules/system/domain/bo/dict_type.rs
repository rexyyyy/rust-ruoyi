use crate::modules::system::domain::SysDictType;
use crate::modules::system::domain::PageRequest;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypePageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
}

impl From<DictTypePageDTO> for PageRequest {
    fn from(arg: DictTypePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&DictTypePageDTO> for PageRequest {
    fn from(arg: &DictTypePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeAddDTO {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>
}

impl From<DictTypeAddDTO> for SysDictType {
    fn from(arg: DictTypeAddDTO) -> Self {
        SysDictType {
            dict_id: crate::util::idseq::next_id() as i64,
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            create_by: None,
            create_time: chrono::Local::now().naive_local() .into(),
            update_by: None,
            update_time: None,
            remark: arg.remark
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictTypeUpdateDTO {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

impl From<DictTypeUpdateDTO> for SysDictType {
    fn from(arg: DictTypeUpdateDTO) -> Self {
        SysDictType {
            dict_id: arg.dict_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
            dict_name: arg.dict_name,
            dict_type: arg.dict_type,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: chrono::Local::now().naive_local() .into(),
            remark: arg.remark,
        }
    }
}
