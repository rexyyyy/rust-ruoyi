use crate::modules::system::domain::{SysDept};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysDeptVO {
    pub dept_id: Option<String>,
    pub parent_id: Option<String>,
    // 上级部门名称：前端部门编辑表单（system/dept/index.vue）用 res.data.parentName 回显
    pub parent_name: Option<String>,
    pub ancestors: Option<String>,
    pub dept_name: Option<String>,
    pub order_num: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub del_flag: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl From<SysDept> for SysDeptVO {
    fn from(arg: SysDept) -> Self {
        Self {
            dept_id: Some(arg.dept_id.to_string()),
            parent_id: arg.parent_id.map(|v| v.to_string()),
            parent_name: None,
            ancestors: arg.ancestors,
            dept_name: arg.dept_name,
            order_num: arg.order_num,
            leader: arg.leader.map(|v| v.to_string()),
            phone: arg.phone,
            email: arg.email,
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: arg.create_by.map(|v| v.to_string()),
            create_time: arg.create_time,
            update_by: arg.update_by.map(|v| v.to_string()),
            update_time: None,
        }
    }
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptTreeVO {
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub parent_id: Option<String>,
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DeptTreeVO>>,
}

impl From<SysDept> for DeptTreeVO {
    fn from(arg: SysDept) -> Self {
        Self {
            id: Some(arg.dept_id.to_string()),
            parent_id: arg.parent_id.map(|v| v.to_string()),
            label: arg.dept_name,
            children: None,
        }
    }
}
impl DeptTreeVO {
    pub fn is_parent(&self) -> bool {
        self.parent_id.is_none() || self.parent_id.clone().unwrap_or_default().eq("0")
    }
}
