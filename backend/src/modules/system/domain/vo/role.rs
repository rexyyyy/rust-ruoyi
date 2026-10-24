use crate::modules::system::domain::SysRole;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysRoleVO {
    pub admin:bool,
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub role_sort: Option<u32>,
    pub data_scope: Option<String>,
    pub menu_check_strictly: Option<bool>,
    pub dept_check_strictly: Option<bool>,
    pub status: Option<String>,
    pub del_flag: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub remark: Option<String>,
    pub flag:bool
}

impl From<SysRole> for SysRoleVO {
    fn from(arg: SysRole) -> Self {
        Self {
            admin: arg.role_id == 1,
            role_id: Some(arg.role_id.to_string()),
            role_name: arg.role_name,
            role_key: arg.role_key,
            role_sort: arg.role_sort.map(|v| v as u32),
            data_scope: arg.data_scope,
            menu_check_strictly: arg.menu_check_strictly,
            dept_check_strictly: arg.dept_check_strictly,
            status: arg.status,
            del_flag: arg.del_flag,
            create_by: arg.create_by.map(|v| v.to_string()),
            create_time: arg.create_time,
            update_by: arg.update_by.map(|v| v.to_string()),
            update_time: arg.update_time,
            remark: arg.remark,
            flag: false,
        }
    }
}

impl SysRoleVO {
    pub fn from_option(arg: Option<SysRole>) -> Option<SysRoleVO> {
        match arg {
            Some(arg) => Some(SysRoleVO {
                admin: arg.role_id == 1,
                role_id: Some(arg.role_id.to_string()),
                role_name: arg.role_name,
                role_key: arg.role_key,
                role_sort: arg.role_sort.map(|v| v as u32),
                data_scope: arg.data_scope,
                menu_check_strictly: arg.menu_check_strictly,
                dept_check_strictly: arg.dept_check_strictly,
                status: arg.status,
                del_flag: arg.del_flag,
                create_by: arg.create_by.map(|v| v.to_string()),
                create_time: arg.create_time,
                update_by: arg.update_by.map(|v| v.to_string()),
                update_time: arg.update_time,
                remark: arg.remark,
                flag: false,
            }),
            _ => None,
        }
    }
}
