use crate::modules::system::domain::{SysUser};
use serde::{Deserialize, Serialize};
use crate::config::global_variables::ADMIN_NAME;
use crate::modules::system::domain::vo::SysRoleVO;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysUserVO {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub password: Option<String>,
    pub status: Option<String>,
    pub del_flag: Option<String>,
    pub login_ip: Option<String>,
    pub login_date: Option<chrono::NaiveDateTime>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub remark: Option<String>,
    pub admin: bool,
    pub roles: Option<Vec<SysRoleVO>>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            user_id: Some(arg.user_id.to_string()),
            dept_id: arg.dept_id.map(|v| v.to_string()),
            user_name: arg.user_name.clone(),
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: arg.avatar.map(|v| v.to_string()),
            //屏蔽密码
            password: None,
            status: arg.status,
            del_flag: arg.del_flag,
            login_ip: arg.login_ip,
            login_date: arg.login_date,
            create_by: arg.create_by.map(|v| v.to_string()),
            create_time: arg.create_time,
            update_by: arg.update_by.map(|v| v.to_string()),
            update_time: arg.update_time,
            remark: arg.remark,
            admin: arg.user_name.unwrap_or_default().eq(ADMIN_NAME),
            roles: None,
        }
    }
}

