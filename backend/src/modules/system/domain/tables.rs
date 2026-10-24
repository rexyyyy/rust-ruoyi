//! sea-orm 实体层（替代原 rbatis 的 `crud!` 表定义）。
//!
//! 每个表一个 `pub mod`，内含 `Model`(数据行) / `Entity`(查询入口) / `ActiveModel` /
//! `Column` / `Relation`。为兼容原调用点：
//! - `pub use <mod>::Model as SysXxx;`  —— `SysXxx` 仍是数据/返回类型
//! - `pub use <mod>::Entity as SysXxxEntity;` —— 查询入口（`SysXxxEntity::find()` 等）
//!
//! 类型对齐官方 `postgres_ry_vue_5.X.sql`：
//! - 主键/外键/审计列（create_by/update_by/create_dept/leader）= i64（PG bigint）
//! - `menu_check_strictly`/`dept_check_strictly` = bool（PG boolean）
//! - 计数列 = i32（PG integer，PG 无 unsigned）
//! - 5.X 的 sys_dict_type / sys_dict_data 无 `status` 列；sys_menu 的查询参数列为 `query_param`
//! - 例外：`sys_trash` 为本项目自建表（官方 SQL 之外的 Rust 回收站功能），id 用 varchar


// ===================== sys_menu =====================
pub mod sys_menu {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_menu")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub menu_id: i64,
        pub menu_name: Option<String>,
        pub parent_id: Option<i64>,
        pub order_num: Option<i32>,
        pub path: Option<String>,
        pub component: Option<String>,
        pub query_param: Option<String>,
        pub is_frame: Option<String>,
        pub is_cache: Option<String>,
        pub menu_type: Option<String>,
        pub visible: Option<String>,
        pub status: Option<String>,
        pub perms: Option<String>,
        pub icon: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_menu::Model as SysMenu;
pub use sys_menu::Entity as SysMenuEntity;
pub use sys_menu::ActiveModel as SysMenuActiveModel;
pub use sys_menu::Column as SysMenuColumn;

// ===================== sys_role =====================
pub mod sys_role {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_role")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub role_id: i64,
        pub role_name: Option<String>,
        pub role_key: Option<String>,
        pub role_sort: Option<i32>,
        pub data_scope: Option<String>,
        pub menu_check_strictly: Option<bool>,
        pub dept_check_strictly: Option<bool>,
        pub status: Option<String>,
        pub del_flag: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_role::Model as SysRole;
pub use sys_role::Entity as SysRoleEntity;
pub use sys_role::ActiveModel as SysRoleActiveModel;
pub use sys_role::Column as SysRoleColumn;

// ===================== sys_role_menu =====================
pub mod sys_role_menu {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, Eq, Hash, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_role_menu")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub role_id: i64,
        #[sea_orm(primary_key, auto_increment = false)]
        pub menu_id: i64,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_role_menu::Model as SysRoleMenu;
pub use sys_role_menu::Entity as SysRoleMenuEntity;
pub use sys_role_menu::ActiveModel as SysRoleMenuActiveModel;
pub use sys_role_menu::Column as SysRoleMenuColumn;

// ===================== sys_user =====================
pub mod sys_user {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_user")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: i64,
        pub dept_id: Option<i64>,
        pub user_name: Option<String>,
        pub nick_name: Option<String>,
        pub email: Option<String>,
        pub phonenumber: Option<String>,
        pub sex: Option<String>,
        pub avatar: Option<i64>,
        pub password: Option<String>,
        pub status: Option<String>,
        pub del_flag: Option<String>,
        pub login_ip: Option<String>,
        pub login_date: Option<NaiveDateTime>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_user::Model as SysUser;
pub use sys_user::Entity as SysUserEntity;
pub use sys_user::ActiveModel as SysUserActiveModel;
pub use sys_user::Column as SysUserColumn;

// ===================== sys_user_role =====================
pub mod sys_user_role {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, Eq, Hash, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_user_role")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: i64,
        #[sea_orm(primary_key, auto_increment = false)]
        pub role_id: i64,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_user_role::Model as SysUserRole;
pub use sys_user_role::Entity as SysUserRoleEntity;
pub use sys_user_role::ActiveModel as SysUserRoleActiveModel;
pub use sys_user_role::Column as SysUserRoleColumn;

// ===================== sys_dept =====================
pub mod sys_dept {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_dept")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub dept_id: i64,
        pub parent_id: Option<i64>,
        pub ancestors: Option<String>,
        pub dept_name: Option<String>,
        pub order_num: Option<i32>,
        pub leader: Option<i64>,
        pub phone: Option<String>,
        pub email: Option<String>,
        pub status: Option<String>,
        pub del_flag: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_dept::Model as SysDept;
pub use sys_dept::Entity as SysDeptEntity;
pub use sys_dept::ActiveModel as SysDeptActiveModel;
pub use sys_dept::Column as SysDeptColumn;

// ===================== sys_dict_type =====================
pub mod sys_dict_type {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_dict_type")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub dict_id: i64,
        pub dict_name: Option<String>,
        pub dict_type: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_dict_type::Model as SysDictType;
pub use sys_dict_type::Entity as SysDictTypeEntity;
pub use sys_dict_type::ActiveModel as SysDictTypeActiveModel;
pub use sys_dict_type::Column as SysDictTypeColumn;

// ===================== sys_dict_data =====================
pub mod sys_dict_data {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_dict_data")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub dict_code: i64,
        pub dict_sort: Option<i32>,
        pub dict_label: Option<String>,
        pub dict_value: Option<String>,
        pub dict_type: Option<String>,
        pub css_class: Option<String>,
        pub list_class: Option<String>,
        pub is_default: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_dict_data::Model as SysDictData;
pub use sys_dict_data::Entity as SysDictDataEntity;
pub use sys_dict_data::ActiveModel as SysDictDataActiveModel;
pub use sys_dict_data::Column as SysDictDataColumn;

// ===================== sys_config =====================
pub mod sys_config {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_config")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub config_id: i64,
        pub config_name: Option<String>,
        pub config_key: Option<String>,
        pub config_value: Option<String>,
        pub config_type: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_config::Model as SysConfig;
pub use sys_config::Entity as SysConfigEntity;
pub use sys_config::ActiveModel as SysConfigActiveModel;
pub use sys_config::Column as SysConfigColumn;

// ===================== sys_notice =====================
pub mod sys_notice {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_notice")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub notice_id: i64,
        pub notice_title: Option<String>,
        pub notice_content: Option<String>,
        pub notice_type: Option<String>,
        pub status: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_notice::Model as SysNotice;
pub use sys_notice::Entity as SysNoticeEntity;
pub use sys_notice::ActiveModel as SysNoticeActiveModel;
pub use sys_notice::Column as SysNoticeColumn;

// ===================== sys_logininfor =====================
pub mod sys_logininfor {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_logininfor")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub info_id: i64,
        pub user_name: Option<String>,
        pub ipaddr: Option<String>,
        pub login_location: Option<String>,
        pub browser: Option<String>,
        pub os: Option<String>,
        pub status: Option<String>,
        pub msg: Option<String>,
        pub login_time: Option<NaiveDateTime>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_logininfor::Model as SysLogininfor;
pub use sys_logininfor::Entity as SysLogininforEntity;
pub use sys_logininfor::ActiveModel as SysLogininforActiveModel;
pub use sys_logininfor::Column as SysLogininforColumn;

// ===================== sys_post =====================
pub mod sys_post {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_post")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub post_id: i64,
        pub post_code: Option<String>,
        pub post_name: Option<String>,
        pub post_sort: Option<i32>,
        pub status: Option<String>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
        pub remark: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_post::Model as SysPost;
pub use sys_post::Entity as SysPostEntity;
pub use sys_post::ActiveModel as SysPostActiveModel;
pub use sys_post::Column as SysPostColumn;

// ===================== sys_trash =====================
// 本项目自建表（官方 SQL 之外），id 用 uuid 字符串
pub mod sys_trash {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_trash")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub table_name: Option<String>,
        pub data: Option<String>,
        pub create_time: Option<NaiveDateTime>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_trash::Model as SysTrash;
pub use sys_trash::Entity as SysTrashEntity;
pub use sys_trash::ActiveModel as SysTrashActiveModel;
pub use sys_trash::Column as SysTrashColumn;

// ===================== sys_role_dept =====================
pub mod sys_role_dept {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, Eq, Hash, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_role_dept")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub role_id: i64,
        #[sea_orm(primary_key, auto_increment = false)]
        pub dept_id: i64,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_role_dept::Model as SysRoleDept;
pub use sys_role_dept::Entity as SysRoleDeptEntity;
pub use sys_role_dept::ActiveModel as SysRoleDeptActiveModel;
pub use sys_role_dept::Column as SysRoleDeptColumn;

// ===================== sys_oper_log =====================
pub mod sys_oper_log {
    use sea_orm::entity::prelude::*;
    use chrono::NaiveDateTime;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "sys_oper_log")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub oper_id: i64,
        pub title: Option<String>,
        pub business_type: Option<i32>,
        pub method: Option<String>,
        pub request_method: Option<String>,
        pub operator_type: Option<i32>,
        pub oper_name: Option<String>,
        pub dept_name: Option<String>,
        pub oper_url: Option<String>,
        pub oper_ip: Option<String>,
        pub oper_location: Option<String>,
        pub oper_param: Option<String>,
        pub json_result: Option<String>,
        pub status: Option<i32>,
        pub error_msg: Option<String>,
        pub oper_time: Option<NaiveDateTime>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub use sys_oper_log::Model as SysOperLog;
pub use sys_oper_log::Entity as SysOperLogEntity;
pub use sys_oper_log::ActiveModel as SysOperLogActiveModel;
pub use sys_oper_log::Column as SysOperLogColumn;

// ===================== sys_dict_type_simple（非表，轻量结构） =====================
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictTypeSimple {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
}
