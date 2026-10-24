use crate::modules::system::domain::table::*;
crud!(SysDept {});


impl_select!(SysDept{select_all_query( dto: &crate::modules::system::domain::dto::dept:: DeptQueryDTO) =>
  "`where parent_id IS NOT NULL`
    if dto.deptName != '' && dto.deptName != None:
      ` and dept_name like #{'%'+dto.deptName+'%'}`
    if dto.status != '' && dto.status != None:
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by order_num`"});

