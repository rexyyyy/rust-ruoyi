use crate::modules::system::domain::table::*;
crud!(SysDictType {});

impl_select_page!(SysDictType{select_page(limit_sql:&str,dto: &crate::modules::system::domain::dto::dict_type::DictTypePageDTO) =>
    "`where 1=1 `
    if dto.dictType != '' && dto.dictType != None:
      ` and dict_type like #{'%'+dto.dictType+'%'}`
    if dto.dictName != '' && dto.dictName != None:
      ` and dict_name like #{'%'+dto.dictName+'%'}`
    if dto.status != '' && dto.status != None:
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by create_time`"});
