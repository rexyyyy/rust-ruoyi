use crate::modules::system::domain::dto::role::RolePageDTO;
use crate::modules::system::domain::table::*;
crud!(SysRole {});
impl_select_page!(SysRole{select_page(limit_sql:&str,dto:&RolePageDTO)=>
    "`where del_flag = '0'`
    if dto.roleName != '' && dto.roleName != None:
      ` and role_name like #{'%'+dto.roleName+'%'}`
    if dto.roleKey != '' && dto.roleKey != None:
      ` and role_key like #{'%'+dto.roleKey+'%'}`
    if dto.status != '' && dto.status != None:
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by role_sort`"});

