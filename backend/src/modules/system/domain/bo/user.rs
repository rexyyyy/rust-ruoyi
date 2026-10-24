use crate::modules::system::domain::{SysUser, SysUserRole};
use crate::modules::system::domain::PageRequest;
use crate::util::password_encoder::PasswordEncoder;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::config::global_variables::{DEL_FLAG_NORMAL, STATUS_NORMAL};

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAddDTO {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub password: Option<String>,
    pub remark: Option<String>,
    pub role_ids: Option<Vec<String>>,
    pub post_ids: Option<Vec<String>>,
}

impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            user_id: crate::util::idseq::next_id() as i64,
            dept_id: arg.dept_id.and_then(|v| v.parse::<i64>().ok()),
            user_name: arg.user_name.clone(),
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: None,
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            status: Some(STATUS_NORMAL.to_string()),
            del_flag: Some(DEL_FLAG_NORMAL.to_string()),
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: chrono::Local::now().naive_local() .into(),
            update_by: None,
            update_time: None,
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDTO {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub status: Option<String>,
    pub password: Option<String>,
    pub remark: Option<String>,
    pub role_ids: Option<Vec<String>>,
    pub post_ids: Option<Vec<String>>,
}

impl From<UserUpdateDTO> for SysUser {
    fn from(arg: UserUpdateDTO) -> Self {
        SysUser {
            user_id: arg.user_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
            dept_id: arg.dept_id.and_then(|v| v.parse::<i64>().ok()),
            user_name: arg.user_name,
            nick_name: arg.nick_name,
            email: arg.email,
            phonenumber: arg.phonenumber,
            sex: arg.sex,
            avatar: None,
            password: None,
            status: arg.status,
            del_flag: None,
            login_ip: None,
            login_date: None,
            create_by: None,
            create_time: None,
            update_by: None,
            update_time: chrono::Local::now().naive_local() .into(),
            remark: arg.remark,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>,
    pub status: Option<String>,
    pub dept_id: Option<String>,
}

impl From<&UserPageDTO> for PageRequest {
    fn from(arg: &UserPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

impl From<&UserRolePageDTO> for UserPageDTO {
    fn from(arg: &UserRolePageDTO) -> Self {
        Self {
            page_no: arg.page_no.clone(),
            page_size: arg.page_size.clone(),
            user_name: arg.user_name.clone(),
            phonenumber: None,
            status: None,
            dept_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleDTO {
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

impl From<UserRoleDTO> for SysUserRole {
    fn from(arg: UserRoleDTO) -> Self {
        SysUserRole {
            user_id: arg.user_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
            role_id: arg.role_id.unwrap_or_default().parse::<i64>().unwrap_or(0),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsersRoleDTO {
    pub user_ids: String,
    pub role_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRolePageDTO {
    #[serde(rename(deserialize = "pageNum"))]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<u64>,
    pub user_name: Option<String>,
    pub name: Option<String>,

}

impl From<&UserRolePageDTO> for PageRequest {
    fn from(arg: &UserRolePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleAuthQueryDTO {
    pub user_id: Option<String>,
    pub role_ids: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PasswordUpdateDTO {
    pub old_password: Option<String>,
    pub new_password: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserIdsQueryDTO {
    pub user_ids: Option<String>,
}