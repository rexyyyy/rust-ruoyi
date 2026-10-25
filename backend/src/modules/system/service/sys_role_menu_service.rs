use crate::modules::system::domain::{
    SysRoleMenu, SysRoleMenuActiveModel, SysRoleMenuColumn, SysRoleMenuEntity,
};
use crate::error::Result;
use crate::pool;
use sea_orm::entity::*;
use sea_orm::query::*;
// DatabaseTransaction 用于把复合写放进调用方已开启的事务
use sea_orm::DatabaseTransaction;
use crate::modules::system::dao::*;

/// Role Menu Service
pub struct SysRoleMenuService {}

impl SysRoleMenuService {
    pub async fn add_role_menus(&self, role_id: i64, menu_ids: Vec<i64>) -> Result<u64> {
        Ok(insert_batch(pool!(), Self::build_role_menus(role_id, menu_ids)).await?)
    }

    /// 事务版 [`Self::add_role_menus`]：与「写角色」同属一个原子操作。
    pub async fn add_role_menus_in_txn(
        &self,
        txn: &DatabaseTransaction,
        role_id: i64,
        menu_ids: Vec<i64>,
    ) -> Result<u64> {
        Ok(insert_batch(txn, Self::build_role_menus(role_id, menu_ids)).await?)
    }

    fn build_role_menus(role_id: i64, menu_ids: Vec<i64>) -> Vec<SysRoleMenuActiveModel> {
        menu_ids
            .into_iter()
            .map(|menu_id| SysRoleMenu { role_id, menu_id }.into_active_model())
            .collect()
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

    /// 按多个 menu_id 一次删除（IN 条件，单条 SQL）。
    /// 供菜单级联删除等批量场景使用，避免逐条 DELETE 的 N 次往返。
    pub async fn remove_by_menu_ids(&self, menu_ids: &[i64]) -> Result<u64> {
        if menu_ids.is_empty() {
            return Ok(0);
        }
        Ok(SysRoleMenuEntity::delete_many()
            .filter(SysRoleMenuColumn::MenuId.is_in(menu_ids.to_vec()))
            .exec(pool!())
            .await?
            .rows_affected)
    }

    /// 事务版 [`Self::remove_by_menu_ids`]：在调用方已开启的事务内清理关联。
    pub async fn remove_by_menu_ids_in_txn(
        &self,
        txn: &DatabaseTransaction,
        menu_ids: &[i64],
    ) -> Result<u64> {
        if menu_ids.is_empty() {
            return Ok(0);
        }
        Ok(SysRoleMenuEntity::delete_many()
            .filter(SysRoleMenuColumn::MenuId.is_in(menu_ids.to_vec()))
            .exec(txn)
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

    /// 事务版 [`Self::remove_by_role_id`]：在调用方已开启的事务内删除菜单关联。
    ///
    /// 用于「删角色 + 删菜单关联 + 写回收站」这类复合写；失败时调用方 `?` 传播，
    /// 句柄 drop 自动 rollback，不会留下孤儿关联行。
    pub async fn remove_by_role_id_in_txn(
        &self,
        txn: &DatabaseTransaction,
        role_id: i64,
    ) -> Result<u64> {
        Ok(SysRoleMenuEntity::delete_many()
            .filter(SysRoleMenuColumn::RoleId.eq(role_id))
            .exec(txn)
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
