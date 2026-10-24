use rbatis::executor::Executor;
use crate::modules::system::domain::dto::role::{RoleAuthUserPageDTO};
use crate::modules::system::domain::dto::user::{ UserPageDTO};
use crate::modules::system::domain::table::*;
use crate::rbatis::rbdc::Error;
crud!(SysUser {});

impl_select_page!(SysUser{select_page(limit_sql:&str,dto:&UserPageDTO)=>
    "`where del_flag = '0'`
    if dto.userName != '' && dto.userName != None:
      ` and user_name like #{'%'+dto.userName+'%'}`
    if dto.phonenumber != '' && dto.phonenumber != None:
      ` and phonenumber like #{'%'+dto.phonenumber+'%'}`
    if dto.status != '' && dto.status != None:
      ` and status = #{dto.status}`
    if dto.deptId != '' && dto.deptId != None:
      ` and (dept_id = #{dto.deptId} OR dept_id IN ( SELECT t.dept_id FROM sys_dept t WHERE find_in_set(#{dto.deptId}, ancestors)))`
    if !sql.contains('count'):
     ` order by create_time`"});

     #[cfg(feature = "postgresql")]
     #[py_sql(
     "`select distinct u.user_id, u.dept_id, u.user_name, u.nick_name, u.email, u.phonenumber, u.status, u.create_time`
         ` from sys_user u`
         ` left join sys_dept d on u.dept_id = d.dept_id`
         ` left join sys_user_role ur on u.user_id = ur.user_id`
         ` left join sys_role r on r.role_id = ur.role_id`
         ` where u.del_flag = '0' and r.role_id = #{dto.roleId}`
       if dto.userName != '' && dto.userName != None:
         ` AND u.user_name like concat('%', #{dto.userName}, '%')`
       if dto.phonenumber != '' && dto.phonenumber != None:
         ` AND u.phonenumber like concat('%', #{dto.phonenumber}, '%')`
       ` limit #{dto.pageSize} offset #{dto.pageoffset}`
     "
     )]
     pub async fn db_auth_user_list(rb: &mut dyn Executor, dto: &RoleAuthUserPageDTO) -> Result<Vec<SysUser>, Error> {
         impled!()
     }

#[cfg(feature = "mysql")]
#[py_sql(
"`select distinct u.user_id, u.dept_id, u.user_name, u.nick_name, u.email, u.phonenumber, u.status, u.create_time`
    ` from sys_user u`
    ` left join sys_dept d on u.dept_id = d.dept_id`
    ` left join sys_user_role ur on u.user_id = ur.user_id`
    ` left join sys_role r on r.role_id = ur.role_id`
    ` where u.del_flag = '0' and r.role_id = #{dto.roleId}`
  if dto.userName != '':
    ` AND u.user_name like concat('%', #{dto.userName}, '%')`
  if dto.phonenumber != '':
    ` AND u.phonenumber like concat('%', #{dto.phonenumber}, '%')`
  ` limit #{dto.pageNo}, #{dto.pageSize}`
"
)]
pub async fn db_auth_user_list(rb: &mut dyn Executor, dto: &RoleAuthUserPageDTO) -> Result<Vec<SysUser>, Error> {
    impled!()
}
#[cfg(feature = "postgresql")]
#[py_sql(
  "`select distinct u.user_id, u.dept_id, u.user_name, u.nick_name, u.email, u.phonenumber, u.status, u.create_time`
   ` from sys_user u`
   ` left join sys_dept d on u.dept_id = d.dept_id`
   ` left join sys_user_role ur on u.user_id = ur.user_id`
   ` left join sys_role r on r.role_id = ur.role_id`
   ` where u.del_flag = '0' and (r.role_id != #{dto.roleId} or r.role_id IS NULL)`
   ` and u.user_id not in (select u.user_id from sys_user u inner join sys_user_role ur on u.user_id = ur.user_id and ur.role_id = #{dto.roleId})`
    if dto.userName != '' && dto.userName != None:
      ` AND u.user_name like concat('%', #{dto.userName}, '%')`
    if dto.phonenumber != '' && dto.phonenumber != None:
      ` AND u.phonenumber like concat('%', #{dto.phonenumber}, '%')`
    ` limit #{dto.pageSize} offset #{dto.pageoffset}`
  "
  )]
  pub async fn db_unallocated_user_list(rb: &mut dyn Executor, dto: &RoleAuthUserPageDTO) -> Result<Vec<SysUser>, Error> {
      impled!()
  }

#[cfg(feature = "mysql")]
#[py_sql(
"`select distinct u.user_id, u.dept_id, u.user_name, u.nick_name, u.email, u.phonenumber, u.status, u.create_time`
 ` from sys_user u`
 ` left join sys_dept d on u.dept_id = d.dept_id`
 ` left join sys_user_role ur on u.user_id = ur.user_id`
 ` left join sys_role r on r.role_id = ur.role_id`
 ` where u.del_flag = '0' and (r.role_id != #{dto.roleId} or r.role_id IS NULL)`
 ` and u.user_id not in (select u.user_id from sys_user u inner join sys_user_role ur on u.user_id = ur.user_id and ur.role_id = #{dto.roleId})`
  if dto.userName != '':
    ` AND u.user_name like concat('%', #{dto.userName}, '%')`
  if dto.phonenumber != '':
    ` AND u.phonenumber like concat('%', #{dto.phonenumber}, '%')`
  ` limit #{dto.pageNo}, #{dto.pageSize}`
"
)]
pub async fn db_unallocated_user_list(rb: &mut dyn Executor, dto: &RoleAuthUserPageDTO) -> Result<Vec<SysUser>, Error> {
    impled!()
}