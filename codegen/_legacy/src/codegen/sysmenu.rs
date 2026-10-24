use rbatis::{crud, rbdc::datetime::DateTime};

//SysMenu
crud!(SysMenu {});//如何去掉第一个AND

///Permission Menu Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysMenu {
    pub menu_id: Option<u64>,
    pub menu_name: Option<String>,
    //父id(可空)
    pub parent_id: Option<u64>,
    //顺序
    pub order_num: Option<u32>,
    //前端-菜单路径
    pub path: Option<String>,
    //组件路径
    pub component: Option<String>,
    //组件路径
    pub query: Option<String>,
    //是否为外链
    pub is_frame: Option<char>,
    //是否缓存
    pub is_cache: Option<char>,
    //菜单类型
    pub menu_type: Option<char>,
    //菜单可见
    pub visible: Option<char>,
    //菜单状态
    pub status: Option<char>,
    //权限标识
    pub perms: Option<String>,
    //图标
    pub icon: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}