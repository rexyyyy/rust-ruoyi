use crate::modules::system::domain::table::*;
crud!(SysLogininfor {});


impl_select_page!(SysLogininfor{select_page(limit_sql:&str,dto: &crate::modules::system::domain::dto::logininfor::LogininforPageDTO) =>
    "``
      if !sql.contains('count'):
         ` order by login_time desc`"});
