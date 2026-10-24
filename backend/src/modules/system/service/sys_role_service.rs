use crate::modules::system::domain::bo::role::{RolePageDTO, RoleAuthUserPageDTO};
use crate::modules::system::domain::{
    Page, PageRequest, SysRole, SysRoleEntity, SysRoleColumn, SysRoleMenu, SysRoleMenuEntity,
    SysRoleMenuColumn, SysUserRoleEntity, SysUserRoleColumn,
};
use crate::modules::system::domain::vo::{SysRoleVO, SysUserVO};
use crate::error::{Error, Result};
use crate::pool;
use crate::modules::system::service::CONTEXT;
use crate::modules::system::mapper::sys_user::{
    db_auth_user_list, db_unallocated_user_list, db_auth_user_count, db_unallocated_user_count,
};
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use crate::modules::system::dao::*;
const RES_KEY: &'static str = "sys_role:all";

///Role of service
pub struct SysRoleService {}

impl SysRoleService {
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = SysRoleEntity::find();
        select = select.filter(SysRoleColumn::DelFlag.eq("0"));
        if let Some(name) = &arg.role_name {
            if !name.is_empty() {
                select = select.filter(SysRoleColumn::RoleName.contains(name.as_str()));
            }
        }
        if let Some(key) = &arg.role_key {
            if !key.is_empty() {
                select = select.filter(SysRoleColumn::RoleKey.contains(key.as_str()));
            }
        }
        if let Some(status) = &arg.status {
            select = select.filter(SysRoleColumn::Status.eq(status.to_string()));
        }
        select = select.order_by_asc(SysRoleColumn::RoleSort);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysRoleVO>())
    }
    ///role details
    pub async fn detail(&self, role_id: &str) -> Result<SysRoleVO> {
        let role = SysRoleEntity::find()
            .filter(SysRoleColumn::RoleId.eq(role_id.parse::<i64>().unwrap_or(0)))
            .all(pool!())
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", role_id)))?;
        let role_vo = SysRoleVO::from(role);
        return Ok(role_vo);
    }

    pub async fn update_cache(&self) -> Result<Vec<SysRole>> {
        let all = SysRoleEntity::find().all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        return Ok(all);
    }

    pub async fn add(&self, role: SysRole, menu_ids: Vec<i64>) -> Result<u64> {
        let result = insert_model(pool!(), role.clone()).await?;

        if result > 0 && !menu_ids.is_empty() {
            CONTEXT
                .sys_role_menu_service
                .add_role_menus(role.role_id, menu_ids).await?;
        }
        self.update_cache().await?;
        Ok(result)
    }

    /// 局部更新：只有 DTO 显式提供的字段才覆盖旧值（对齐原 RuoYi `updateRole` 跳过 null 的语义）。
    /// 否则「状态开关」这类只带 roleId/status 的请求会把 role_name 等非空列写成 NULL（MySQL 报 1048）。
    /// `menu_ids` 为 None 表示本次不处理菜单授权，避免状态开关把已有授权清空。
    pub async fn update(&self, role: SysRole, menu_ids: Option<Vec<i64>>) -> Result<u64> {
        let mut merged = SysRoleEntity::find()
            .filter(SysRoleColumn::RoleId.eq(role.role_id))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("角色{}不存在！", role.role_id)))?;

        merged.role_name = role.role_name.or(merged.role_name);
        merged.role_key = role.role_key.or(merged.role_key);
        merged.role_sort = role.role_sort.or(merged.role_sort);
        merged.data_scope = role.data_scope.or(merged.data_scope);
        merged.menu_check_strictly = role.menu_check_strictly.or(merged.menu_check_strictly);
        merged.dept_check_strictly = role.dept_check_strictly.or(merged.dept_check_strictly);
        merged.status = role.status.or(merged.status);
        merged.update_by = role.update_by.or(merged.update_by);
        merged.update_time = role.update_time.or(merged.update_time);
        merged.remark = role.remark.or(merged.remark);

        let result = update_model(pool!(), merged.clone()).await?;
        if result > 0 {
            if let Some(menu_ids) = menu_ids {
                CONTEXT
                    .sys_role_menu_service
                    .remove_by_role_id(merged.role_id)
                    .await?;
                if !menu_ids.is_empty() {
                    CONTEXT
                        .sys_role_menu_service
                        .add_role_menus(merged.role_id, menu_ids)
                        .await?;
                }
            }
        }
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn remove(&self, id: &String) -> Result<u64> {
        let role_id = id.parse::<i64>().unwrap_or(0);
        let trash = SysRoleEntity::find()
            .filter(SysRoleColumn::RoleId.eq(role_id))
            .all(pool!())
            .await?;
        let result = SysRoleEntity::delete_many()
            .filter(SysRoleColumn::RoleId.eq(role_id))
            .exec(pool!())
            .await?
            .rows_affected;
        if result > 0 {
            CONTEXT
                .sys_role_menu_service
                .remove_by_role_id(role_id)
                .await?;
        }
        CONTEXT.sys_trash_service.add("sys_role", &trash).await?;
        self.update_cache().await?;
        Ok(result)
    }

    pub async fn finds(&self, ids: &Vec<i64>) -> Result<Vec<SysRole>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRoleEntity::find()
            .filter(SysRoleColumn::RoleId.is_in(ids.clone()))
            .all(pool!())
            .await?)
    }

    pub async fn finds_all(&self) -> Result<Vec<SysRoleVO>> {
        let data = SysRoleEntity::find().all(pool!()).await?;
        let mut role_vos = vec![];
        for s in data {
            role_vos.push(SysRoleVO::from(s));
        }
        Ok(role_vos)
    }

    //查找所有roles，如果用户包含此权限，则flag=true
    pub async fn finds_roles_by_user_id(&self, user_id: &str) -> Result<Vec<SysRoleVO>> {
        let all = SysRoleEntity::find().all(pool!()).await?;
        let mut res = vec![];
        // 原 rbatis 此处误写成 SysRole::select_by_map("user_id")，无该列；
        // 按注释意图改为查 SysUserRole 取该用户的角色 id。
        let user_roles = SysUserRoleEntity::find()
            .filter(SysUserRoleColumn::UserId.eq(user_id.parse::<i64>().unwrap_or(0)))
            .all(pool!())
            .await?;
        for r in all {
            let mut r_vo = SysRoleVO::from(r);

            for ur in &user_roles {
                if r_vo.role_id.as_deref().and_then(|s| s.parse::<i64>().ok()) == Some(ur.role_id) {
                    r_vo.flag = true;
                }
            }
            res.push(r_vo);
        }

        res.sort_by(|a, b| a.role_sort.cmp(&b.role_sort));

        Ok(res)
    }

    pub async fn find_role_menu(&self, role_ids: &Vec<i64>) -> Result<Vec<SysRoleMenu>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(SysRoleMenuEntity::find()
            .filter(SysRoleMenuColumn::RoleId.is_in(role_ids.clone()))
            .all(pool!())
            .await?)
    }

    pub async fn auth_user_list_page(&self, arg: &RoleAuthUserPageDTO) -> Result<Vec<SysUserVO>> {
        let offset = (arg.page_no.unwrap_or(1) - 1) * arg.page_size.unwrap_or(10);
        let arg: &RoleAuthUserPageDTO = &arg.set_pageoffset(offset);

        let res = db_auth_user_list(pool!(), &arg).await?;
        let mut sys_users = vec![];
        for user in res {
            sys_users.push(SysUserVO::from(user));
        }
        Ok(sys_users)
    }
    pub async fn unallocated_user_list_page(
        &self,
        arg: &RoleAuthUserPageDTO,
    ) -> Result<Vec<SysUserVO>> {
        let offset = (arg.page_no.unwrap_or(1) - 1) * arg.page_size.unwrap_or(10);
        let arg: &RoleAuthUserPageDTO = &arg.set_pageoffset(offset);
        let res = db_unallocated_user_list(pool!(), &arg).await?;
        let mut sys_users = vec![];
        for user in res {
            sys_users.push(SysUserVO::from(user));
        }
        Ok(sys_users)
    }

    /// 已分配该角色的用户总数（供分页 total 使用）
    pub async fn auth_user_list_count(&self, arg: &RoleAuthUserPageDTO) -> Result<u64> {
        db_auth_user_count(pool!(), arg).await
    }

    /// 未分配该角色的用户总数（供分页 total 使用）
    pub async fn unallocated_user_list_count(&self, arg: &RoleAuthUserPageDTO) -> Result<u64> {
        db_unallocated_user_count(pool!(), arg).await
    }
}

impl SysRoleService {
    /// 根据角色ID列表查询角色（对应Spring /system/role/optionselect）
    pub async fn finds_by_ids(&self, role_ids: &Vec<String>) -> Result<Vec<SysRoleVO>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        // PostgreSQL 占位符是 $1,$2...，"?" 是 MySQL 风格，在 PG 下会报
        // "语法错误 在 ) 或附近的"，导致 RoleSelect 组件带 ids 查询直接 500。
        let placeholders = (1..=role_ids.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!("select * from sys_role where role_id in ({})", placeholders);
        let values: Vec<sea_orm::Value> = role_ids.iter().map(|s| Value::from(s.parse::<i64>().unwrap_or(0))).collect();
        let rows = pool!()
            .query_all(crate::db::raw_stmt( sql, values))
            .await?;
        let data: Vec<SysRole> = rows
            .iter()
            .filter_map(|r| SysRole::from_query_result(r, "").ok())
            .collect();
        Ok(data.into_iter().map(SysRoleVO::from).collect())
    }
}
