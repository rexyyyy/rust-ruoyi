use crate::modules::system::domain::bo::role::RoleAuthUserPageDTO;
use crate::modules::system::domain::SysUser;
use crate::error::Result;
use sea_orm::{ConnectionTrait, DbBackend, Statement, Value};
use sea_orm::*;

pub async fn db_auth_user_list(db: &DatabaseConnection, dto: &RoleAuthUserPageDTO) -> Result<Vec<SysUser>> {
    let mut sql = String::from(
        "select distinct u.* from sys_user u \
         left join sys_dept d on u.dept_id = d.dept_id \
         left join sys_user_role ur on u.user_id = ur.user_id \
         left join sys_role r on r.role_id = ur.role_id \
         where u.del_flag = '0' and r.role_id = $1",
    );
    let mut vals: Vec<Value> = vec![Value::from(dto.role_id.clone().unwrap_or_default().parse::<i64>().unwrap_or(0))];
    if let Some(n) = &dto.user_name {
        if !n.is_empty() {
            sql.push_str(" AND u.user_name like $2");
            vals.push(Value::from(format!("%{}%", n)));
        }
    }
    if let Some(p) = &dto.phonenumber {
        if !p.is_empty() {
            sql.push_str(&format!(" AND u.phonenumber like ${}", vals.len() + 1));
            vals.push(Value::from(format!("%{}%", p)));
        }
    }
    sql.push_str(&format!(
        " limit {} offset {}",
        dto.page_size.unwrap_or(10),
        dto.pageoffset.unwrap_or(0)
    ));
    let rows = db
        .query_all(crate::db::raw_stmt( &sql, vals))
        .await?;
    Ok(rows
        .iter()
        .filter_map(|r| SysUser::from_query_result(r, "").ok())
        .collect())
}

pub async fn db_unallocated_user_list(db: &DatabaseConnection, dto: &RoleAuthUserPageDTO) -> Result<Vec<SysUser>> {
    let mut sql = String::from(
        "select distinct u.* from sys_user u \
         left join sys_dept d on u.dept_id = d.dept_id \
         left join sys_user_role ur on u.user_id = ur.user_id \
         left join sys_role r on r.role_id = ur.role_id \
         where u.del_flag = '0' and (r.role_id != $1 or r.role_id IS NULL) \
         and u.user_id not in (select u.user_id from sys_user u inner join sys_user_role ur on u.user_id = ur.user_id and ur.role_id = $1)",
    );
    let mut vals: Vec<Value> = vec![Value::from(dto.role_id.clone().unwrap_or_default().parse::<i64>().unwrap_or(0))];
    if let Some(n) = &dto.user_name {
        if !n.is_empty() {
            sql.push_str(" AND u.user_name like $2");
            vals.push(Value::from(format!("%{}%", n)));
        }
    }
    if let Some(p) = &dto.phonenumber {
        if !p.is_empty() {
            sql.push_str(&format!(" AND u.phonenumber like ${}", vals.len() + 1));
            vals.push(Value::from(format!("%{}%", p)));
        }
    }
    sql.push_str(&format!(
        " limit {} offset {}",
        dto.page_size.unwrap_or(10),
        dto.pageoffset.unwrap_or(0)
    ));
    let rows = db
        .query_all(crate::db::raw_stmt( &sql, vals))
        .await?;
    Ok(rows
        .iter()
        .filter_map(|r| SysUser::from_query_result(r, "").ok())
        .collect())
}

/// 已分配该角色的用户总数（分页 total，where 条件与 db_auth_user_list 保持一致）
pub async fn db_auth_user_count(db: &DatabaseConnection, dto: &RoleAuthUserPageDTO) -> Result<u64> {
    let mut sql = String::from(
        "select count(distinct u.user_id) as total from sys_user u \
         left join sys_dept d on u.dept_id = d.dept_id \
         left join sys_user_role ur on u.user_id = ur.user_id \
         left join sys_role r on r.role_id = ur.role_id \
         where u.del_flag = '0' and r.role_id = $1",
    );
    let mut vals: Vec<Value> = vec![Value::from(dto.role_id.clone().unwrap_or_default().parse::<i64>().unwrap_or(0))];
    if let Some(n) = &dto.user_name {
        if !n.is_empty() {
            sql.push_str(" AND u.user_name like $2");
            vals.push(Value::from(format!("%{}%", n)));
        }
    }
    if let Some(p) = &dto.phonenumber {
        if !p.is_empty() {
            sql.push_str(&format!(" AND u.phonenumber like ${}", vals.len() + 1));
            vals.push(Value::from(format!("%{}%", p)));
        }
    }
    let row = db
        .query_one(crate::db::raw_stmt( &sql, vals))
        .await?;
    Ok(row
        .and_then(|r| r.try_get::<i64>("", "total").ok())
        .unwrap_or(0) as u64)
}

/// 未分配该角色的用户总数（分页 total，where 条件与 db_unallocated_user_list 保持一致）
pub async fn db_unallocated_user_count(db: &DatabaseConnection, dto: &RoleAuthUserPageDTO) -> Result<u64> {
    let mut sql = String::from(
        "select count(distinct u.user_id) as total from sys_user u \
         left join sys_dept d on u.dept_id = d.dept_id \
         left join sys_user_role ur on u.user_id = ur.user_id \
         left join sys_role r on r.role_id = ur.role_id \
         where u.del_flag = '0' and (r.role_id != $1 or r.role_id IS NULL) \
         and u.user_id not in (select u.user_id from sys_user u inner join sys_user_role ur on u.user_id = ur.user_id and ur.role_id = $1)",
    );
    let mut vals: Vec<Value> = vec![Value::from(dto.role_id.clone().unwrap_or_default().parse::<i64>().unwrap_or(0))];
    if let Some(n) = &dto.user_name {
        if !n.is_empty() {
            sql.push_str(" AND u.user_name like $2");
            vals.push(Value::from(format!("%{}%", n)));
        }
    }
    if let Some(p) = &dto.phonenumber {
        if !p.is_empty() {
            sql.push_str(&format!(" AND u.phonenumber like ${}", vals.len() + 1));
            vals.push(Value::from(format!("%{}%", p)));
        }
    }
    let row = db
        .query_one(crate::db::raw_stmt( &sql, vals))
        .await?;
    Ok(row
        .and_then(|r| r.try_get::<i64>("", "total").ok())
        .unwrap_or(0) as u64)
}
