use crate::modules::system::domain::table::*;
crud!(SysConfig {});

// 由于没有输入查询条件时
// 
impl_select_page!(SysConfig{select_page(limit_sql:&str,dto: &crate::modules::system::domain::dto::config:: ConfigPageDTO) =>
    "`where 1=1`
    if dto.configName != '' && dto.configName != None:
      ` and config_name like #{'%'+dto.configName+'%'}`
    if dto.configKey != '' && dto.configKey != None:
      ` and config_key like #{'%'+dto.configKey+'%'}`
    if dto.configType != '' && dto.configType != None:
      ` and config_type = #{dto.configType}`
    if !sql.contains('count'):
     ` order by create_time`"});
