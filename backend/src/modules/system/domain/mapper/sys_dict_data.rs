use crate::modules::system::domain::table::SysDictData;
crud!(SysDictData {});


impl_select_page!(SysDictData{select_page(limit_sql:&str,dto: &crate::modules::system::domain::dto::dict_data:: DictDataPageDTO) =>
    "`where 1=1 `
    if dto.dictType != '' && dto.dictType != None:
      ` and dict_type = #{dto.dictType}`
    if dto.dictLabel != '' && dto.dictLabel != None:
      ` and dict_label like #{'%'+dto.dictLabel+'%'}`
    if dto.status != '' && dto.status != None:
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by dict_sort`"});
impl_select!(SysDictData{select_by_dict_type(dict_type:&String)=>"`where dict_type =#{dict_type} order by dict_sort`"});
