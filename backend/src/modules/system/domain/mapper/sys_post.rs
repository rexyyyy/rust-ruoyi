use crate::modules::system::domain::table::*;
crud!(SysPost {});


impl_select_page!(SysPost{select_page(limit_sql:&str,dto: &crate::modules::system::domain::dto::post::PostPageDTO) =>
    "`where 1=1 `
    if dto.postName != '' && dto.postName != None:
      ` and post_name like #{'%'+dto.postName+'%'}`
    if dto.postCode != '' && dto.postCode != None:
      ` and post_code like #{'%'+dto.postCode+'%'}`
    if dto.status != '' && dto.status != None:
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by post_sort`"});