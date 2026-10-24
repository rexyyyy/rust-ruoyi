use crate::modules::system::domain::table::*;
use crate::modules::system::domain::dto::menu::*;
//SysMenu
crud!(SysMenu {});//如何去掉第一个AND
impl_select!(SysMenu{query_menu(dto: &MenuPageDTO) =>
"`where 1=1`
    if dto.menuName != '' && dto.menuName != None:
      ` and menu_name like #{'%'+dto.menuName+'%'}`
    if dto.status != '' && dto.status != None:
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by order_num`"});
impl_select!(SysMenu{select_all_order_num() =>
    "` order by order_num`"});