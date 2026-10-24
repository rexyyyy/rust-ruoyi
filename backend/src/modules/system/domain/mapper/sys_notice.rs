use crate::modules::system::domain::table::*;
crud!(SysNotice {});


impl_select_page!(SysNotice{select_page(limit_sql:&str,dto: &crate::modules::system::domain::dto::notice::NoticePageDTO) =>
    "`where 1=1`
    if dto.noticeTitle != '' && dto.noticeTitle != None:
      ` and notice_title like #{'%'+dto.noticeTitle+'%'}`
    if dto.createBy != '' && dto.createBy != None:
      ` and create_by = #{dto.createBy}`
    if dto.noticeType != '' && dto.noticeType != None:
      ` and notice_type = #{dto.noticeType}`
    if !sql.contains('count'):
     ` order by notice_title`"});

