use std::collections::BTreeMap;

use crate::modules::system::domain::bo::user::{UserPageDTO, UserRoleDTO, UserRolePageDTO};
use crate::modules::system::domain::{SysRole, SysUserRole, SysUserRoleColumn, SysUserRoleEntity};
use crate::modules::system::domain::vo::user::SysUserVO;
use crate::modules::system::domain::vo::{SysMenuVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;

use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use crate::modules::system::domain::Page;
use crate::modules::system::dao::*;

///User Role Service
pub struct SysUserRoleService {}

impl SysUserRoleService {
    pub async fn page(&self, arg: &UserRolePageDTO) -> Result<Page<SysUserVO>> {
        let vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(arg))
            .await?;
        return Ok(vo);
    }

    pub async fn add(&self, arg: UserRoleDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let user_id = arg.user_id.as_deref().unwrap().to_string();
        let user_role = SysUserRole::from(arg);
        self.remove_by_user_id(user_id.as_str()).await?;
        Ok(insert_model(pool!(), user_role).await?)
    }


    pub async fn add_user_roles(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        let rows = role_ids.iter().map(|r_id| SysUserRole {
            user_id: user_id.parse::<i64>().unwrap_or(0),
            role_id: r_id.parse::<i64>().unwrap_or(0),
        }).collect::<Vec<_>>();

        Ok(insert_batch(pool!(), rows.iter().cloned().map(|m| m.into_active_model()).collect()).await?)
    }

    pub async fn add_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids.iter().map(|u_id| SysUserRole {
            user_id: u_id.parse::<i64>().unwrap_or(0),
            role_id: role_id.parse::<i64>().unwrap_or(0),
        }).collect::<Vec<_>>();

        Ok(insert_batch(pool!(), rows.iter().cloned().map(|m| m.into_active_model()).collect()).await?)
    }


    pub async fn remove(&self, user_role: &SysUserRole) -> Result<u64> {
        let res = pool!()
            .execute(crate::db::raw_stmt(
                                "delete from sys_user_role where user_id = $1 and role_id = $2",
                vec![
                    Value::from(user_role.user_id.clone()),
                    Value::from(user_role.role_id.clone()),
                ],
            ))
            .await?;
        Ok(res.rows_affected())
    }
    pub async fn remove_users_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        let rows = user_ids.iter().map(|u_id| SysUserRole {
            user_id: u_id.parse::<i64>().unwrap_or(0),
            role_id: role_id.parse::<i64>().unwrap_or(0),
        }).collect::<Vec<_>>();

        let mut cnt = 0;
        for r in rows {
            let res = self.remove(&r).await;
            cnt = cnt + res.unwrap();
        }
        Ok(cnt)
    }
    pub async fn remove_by_role_id(&self, role_id: &String) -> Result<u64> {
        Ok(
            SysUserRoleEntity::delete_many()
                .filter(SysUserRoleColumn::RoleId.eq(role_id.parse::<i64>().unwrap_or(0)))
                .exec(pool!())
                .await?
                .rows_affected,
        )
    }


    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(
            SysUserRoleEntity::delete_many()
                .filter(SysUserRoleColumn::UserId.eq(user_id.parse::<i64>().unwrap_or(0)))
                .exec(pool!())
                .await?
                .rows_affected,
        )
    }

    /// 事务版 [`Self::remove_by_user_id`]：在调用方已开启的事务内删除角色关联。
    ///
    /// 用于「删用户 + 删角色关联」这类必须原子的复合写；失败时调用方 `?` 传播，
    /// 句柄 drop 自动 rollback，不会留下孤儿的角色关联行。
    pub async fn remove_by_user_id_in_txn(
        &self,
        txn: &DatabaseTransaction,
        user_id: &str,
    ) -> Result<u64> {
        Ok(
            SysUserRoleEntity::delete_many()
                .filter(SysUserRoleColumn::UserId.eq(user_id.parse::<i64>().unwrap_or(0)))
                .exec(txn)
                .await?
                .rows_affected,
        )
    }


    pub async fn reset_through_user_id(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        SysUserRoleEntity::delete_many()
            .filter(SysUserRoleColumn::UserId.eq(user_id.parse::<i64>().unwrap_or(0)))
            .exec(pool!())
            .await?;
        self.add_user_roles(user_id, role_ids).await
    }

    /// 事务版 [`Self::add_user_roles`]：与「新增用户」同属一个原子操作。
    pub async fn add_user_roles_in_txn(
        &self,
        txn: &DatabaseTransaction,
        user_id: &str,
        role_ids: &[String],
    ) -> Result<u64> {
        let rows = role_ids
            .iter()
            .map(|r_id| SysUserRole {
                user_id: user_id.parse::<i64>().unwrap_or(0),
                role_id: r_id.parse::<i64>().unwrap_or(0),
            })
            .map(|m| m.into_active_model())
            .collect::<Vec<_>>();
        Ok(insert_batch(txn, rows).await?)
    }

    /// 事务版 [`Self::reset_through_user_id`]：「先删后插」重置角色必须原子。
    ///
    /// 旧实现删与插走两条独立连接，中间失败会让用户一旦失去角色（永久丢权限），
    /// 或半插成功留下不完整的角色集合。
    pub async fn reset_through_user_id_in_txn(
        &self,
        txn: &DatabaseTransaction,
        user_id: &str,
        role_ids: &Vec<String>,
    ) -> Result<u64> {
        SysUserRoleEntity::delete_many()
            .filter(SysUserRoleColumn::UserId.eq(user_id.parse::<i64>().unwrap_or(0)))
            .exec(txn)
            .await?;
        let rows = role_ids
            .iter()
            .map(|r_id| SysUserRole {
                user_id: user_id.parse::<i64>().unwrap_or(0),
                role_id: r_id.parse::<i64>().unwrap_or(0),
            })
            .collect::<Vec<_>>();
        Ok(insert_batch(txn, rows.into_iter().map(|m| m.into_active_model()).collect()).await?)
    }
    pub async fn find_roles_by_user_id(
        &self,
        user_id: i64,
        _all_menus: &BTreeMap<i64, SysMenuVO>,
    ) -> Result<Option<Vec<SysRole>>> {
        if user_id == 0 {
            return Ok(None);
        }
        let user_roles = SysUserRoleEntity::find()
            .filter(SysUserRoleColumn::UserId.eq(user_id))
            .all(pool!())
            .await?;

        let role_ids: Vec<i64> = user_roles.iter().map(|x| x.role_id).collect();
        let roles = CONTEXT.sys_role_service.finds(&role_ids).await?;
        Ok(Some(roles))
    }

    //TODO delete
    // pub async fn find_user_role(
    //     &self,
    //     user_id: &str,
    //     all_menus: &BTreeMap<String, SysMenuVO>,
    // ) -> Result<Option<SysRoleVO>> {
    //     if user_id.is_empty() {
    //         return Ok(None);
    //     }
    //     let user_roles =
    //         SysUserRole::select_by_column(pool!(), field_name!(SysUserRole.user_id), user_id)
    //             .await?;
    //
    //     let role_ids = &rbatis::make_table_field_vec!(&user_roles, role_id);
    //     let roles = CONTEXT.sys_role_service.finds(role_ids).await?;
    //     let role_menu_vec = CONTEXT
    //         .sys_role_service
    //         .find_role_menu(&rbatis::make_table_field_vec!(&user_roles, role_id))
    //         .await?;
    //     let mut role_vos = vec![];
    //     for role in roles {
    //         //load res
    //         let mut menus = vec![];
    //         for role_menu in &role_menu_vec {
    //             if role.id.is_some() && role.id.eq(&role_menu.role_id) {
    //                 if let Some(res) = all_menus.get(role_menu.menu_id.as_ref().unwrap_or_def()) {
    //                     menus.push(res.clone());
    //                 }
    //             }
    //         }
    //         let mut vo = SysRoleVO::from(role);
    //         vo.menu_ids = CONTEXT.sys_menu_service.make_menu_ids(&menus);
    //         vo.menus = menus;
    //         role_vos.push(vo);
    //     }
    //     if role_vos.is_empty() {
    //         return Ok(None);
    //     } else {
    //         return Ok(Some(role_vos[0].clone()));
    //     }
    // }
}
