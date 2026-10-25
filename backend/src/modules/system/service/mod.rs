mod cache_service;
mod mem_service;
mod moka_service;
mod redis_service;

mod sys_config_service;
mod sys_dept_service;
mod sys_dict_data_service;
mod sys_dict_type_service;
mod sys_logininfor_service;
mod sys_menu_service;
mod sys_notice_service;
mod sys_operlog_service;
mod sys_post_service;
mod sys_role_menu_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_trash_service;
mod sys_user_role_service;
mod sys_user_service;

pub use crate::config::config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
pub use moka_service::*;
use once_cell::sync::Lazy;
pub use redis_service::*;
pub use sys_config_service::*;
pub use sys_dept_service::*;
pub use sys_dict_data_service::*;
pub use sys_dict_type_service::*;
pub use sys_logininfor_service::*;
pub use sys_menu_service::*;
pub use sys_notice_service::*;
pub use sys_operlog_service::*;
pub use sys_post_service::*;
pub use sys_role_menu_service::*;
pub use sys_role_service::*;
pub use sys_sms_service::*;
pub use sys_trash_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;

use chrono::{Datelike, Local};
use std::sync::Arc;
use sea_orm::{ConnectionTrait, DbBackend, Statement};
/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

/// 全局 sea-orm 连接（替代原 rbatis 的 `pool!()`）。
#[macro_export]
macro_rules! pool {
    () => {
        crate::db::db().expect("[ruoyi_rust] sea-orm db not initialized")
    };
}

#[macro_export]
macro_rules! get_config_value {
    ($arg:expr) => {
        $crate::modules::system::service::CONTEXT
            .sys_config_service
            .select_config_by_key($arg)
            .await
            .unwrap_or_default()
    };
}

pub struct ServiceContext {
    /// 运行配置。与 `crate::state::AppState.config` 共享**同一个 `Arc`**
    /// （数据源均为 `crate::config::config::global()`），全进程只有一份配置数据，
    /// 消除历史上「CONTEXT 一份、AppState 一份」的双轨不一致。
    pub config: Arc<ApplicationConfig>,
    pub cache_service: CacheService,
    pub sys_menu_service: SysMenuService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_menu_service: SysRoleMenuService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_dict_type_service: SysDictTypeService,
    pub sys_dict_data_service: SysDictDataService,
    pub sys_config_service: SysConfigService,
    pub sys_dept_service: SysDeptService,
    pub sys_trash_service: SysTrashService,
    pub sys_logininfor_service: SysLogininforService,
    pub sys_post_service: SysPostService,
    pub sys_notice_service: SysNoticeService,
    pub sys_operlog_service: SysOperlogService,
    pub char_array: [char; 52],
}

impl ServiceContext {
    // 获取业务流水号
    // 注意：idfield/table_name 会被直接拼进 SQL，仅允许安全的标识符（字母/数字/下划线），
    // 防御将来被传入用户可控输入时产生 SQL 注入。
    pub async fn get_table_maxid(idfield: &str, table_name: &str) -> i64 {
        let is_ident = |s: &str| {
            !s.is_empty()
                && s.len() <= 64
                && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        };
        if !is_ident(idfield) || !is_ident(table_name) {
            log::error!(
                "[ruoyi_rust] get_table_maxid rejected unsafe identifier: {}/{}",
                idfield,
                table_name
            );
            return 0;
        }
        let local = Local::now();
        let year = local.year();
        let month = local.month();
        let day = local.day();
        let prefix = format!("{:04}{:02}{:02}", year, month, day);

        let query_perms = format!(
            "select max(CAST({} AS BIGINT)) as max_id from {} where CAST({} AS TEXT) like '{}%'",
            idfield, table_name, idfield, prefix
        );
        let db = crate::db::db().expect("[ruoyi_rust] sea-orm db not initialized");
        let max_id: i64 = match db
            .query_one(crate::db::raw_stmt( query_perms, vec![]))
            .await
        {
            Ok(Some(qr)) => qr.try_get::<i64>("", "max_id").unwrap_or(0),
            _ => 0,
        };

        let newid: i64;
        if max_id <= 0 {
            let new_str = format!("{:04}{:02}{:02}{:05}", year, month, day, 1);
            let s = new_str.to_string();
            //字符串转32位有符号整型
            newid = s.parse::<i64>().unwrap();
        } else {
            newid = max_id + 1;
        }
        newid.into()
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        // 取全局配置单例：**不再**自行加载第二份。`AppState` 会从同一个 Arc 取值，
        // 两者天然一致（改一处即可被另一处看到）。
        let config = crate::config::config::global().clone();
        let str = "abcdefghkmnopqrstuwxyz23456789ABCDEFGHKMNOPQRSTUWXYZ";
        let char_array1: Vec<char> = str.chars().collect();
        let char_array2: [char; 52] = char_array1
            .into_iter()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        let cache_service = CacheService::new(&config).unwrap();
        ServiceContext {
            config,
            cache_service,
            sys_menu_service: SysMenuService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_menu_service: SysRoleMenuService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_type_service: SysDictTypeService {},
            sys_dict_data_service: SysDictDataService {},
            sys_config_service: SysConfigService {},
            sys_dept_service: SysDeptService {},
            sys_trash_service: SysTrashService {},
            sys_logininfor_service: SysLogininforService {},
            sys_post_service: SysPostService {},
            sys_notice_service: SysNoticeService {},
            sys_operlog_service: SysOperlogService {},
            char_array: char_array2,
        }
    }
}
