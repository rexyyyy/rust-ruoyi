use crate::modules::system::domain::{SysRoleMenu, SysRoleMenuColumn, SysRoleMenuEntity};
use crate::error::Result;
use crate::pool;
use sea_orm::entity::*;
use sea_orm::query::*;
use crate::modules::system::dao::*;

/// Role Menu Service
pub struct SysRoleMenuService {}

impl SysRoleMenuService {
    pub async fn add_role_menus(&self, role_id: i64, menu_ids: Vec<i64>) -> Result<u64> {
        let mut sys_role_menu = vec![];
        for menu_id in menu_ids {
            sys_role_menu.push(SysRoleMenu {
                role_id,
                menu_id,
            });
        }
        Ok(insert_batch(
            pool!(),
            sys_role_menu
                .iter()
                .cloned()
                .map(|m| m.into_active_model())
                .collect(),
        )
        .await?)
    }

    pub async fn add_roles_menu(&self, menu_id: i64, role_ids: Vec<i64>) -> Result<u64> {
        let mut sys_role_menus = vec![];
        for role_id in role_ids {
            sys_role_menus.push(SysRoleMenu {
                role_id,
                menu_id,
            });
        }
        Ok(insert_batch(
            pool!(),
            sys_role_menus
                .iter()
                .cloned()
                .map(|m| m.into_active_model())
                .collect(),
        )
        .await?)
    }

    pub async fn remove_by_menu_id(&self, menu_id: i64) -> Result<u64> {
        Ok(SysRoleMenuEntity::delete_many()
            .filter(SysRoleMenuColumn::MenuId.eq(menu_id))
            .exec(pool!())
            .await?
            .rows_affected)
    }

    pub async fn remove_by_role_id(&self, role_id: i64) -> Result<u64> {
        Ok(SysRoleMenuEntity::delete_many()
            .filter(SysRoleMenuColumn::RoleId.eq(role_id))
            .exec(pool!())
            .await?
            .rows_affected)
    }

    pub async fn select_by_role_id(&self, role_id: i64) -> Result<Vec<SysRoleMenu>> {
        Ok(SysRoleMenuEntity::find()
            .filter(SysRoleMenuColumn::RoleId.eq(role_id))
            .all(pool!())
            .await?)
    }
}
