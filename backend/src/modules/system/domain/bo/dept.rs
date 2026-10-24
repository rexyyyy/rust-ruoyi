use crate::modules::system::domain::SysDept;
use serde::{Deserialize, Serialize};
use crate::config::global_variables::{DEL_FLAG_NORMAL, STATUS_NORMAL};

/// dept query DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptQueryDTO {
    pub dept_name: Option<String>,
    pub status: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptAddDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<u16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl From<DeptAddDTO> for SysDept {
    fn from(arg: DeptAddDTO) -> Self {
        SysDept {
            dept_id: crate::util::idseq::next_id(),
            parent_id: arg.parent_id.and_then(|v| v.parse::<i64>().ok()),
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num.map(|v| v as i32),
            leader: arg.leader.and_then(|v| v.parse::<i64>().ok()),
            phone: arg.phone,
            email: arg.email,
            status: Some(STATUS_NORMAL.to_string()),
            del_flag: Some(DEL_FLAG_NORMAL.to_string()),
            create_by: None,
            create_time: chrono::Local::now().naive_local() .into(),
            update_by: None,
            update_time: None,

        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptUpdateDTO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<u16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub del_flag: Option<String>,
}

impl From<DeptUpdateDTO> for SysDept {
    fn from(arg: DeptUpdateDTO) -> Self {
        SysDept {
            dept_id: arg.dept_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
            parent_id: arg.parent_id.and_then(|v| v.parse::<i64>().ok()),
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num.map(|v| v as i32),
            leader: arg.leader.and_then(|v| v.parse::<i64>().ok()),
            phone: arg.phone,
            email: arg.email,
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: chrono::Local::now().naive_local() .into(),
        }
    }
}
