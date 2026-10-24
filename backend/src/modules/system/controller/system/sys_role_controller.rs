use crate::modules::system::domain::bo::role::{RoleAddDTO, RoleUpdateDTO, RolePageDTO, RoleAuthUserPageDTO};
use crate::modules::system::domain::bo::user::{ UserRoleDTO, UsersRoleDTO};

use crate::modules::system::domain::vo::{PageVO, RespJson, RespVO};
use crate::web_data::CurrentUser;
use crate::modules::system::service::CONTEXT;
use futures_util::future::BoxFuture;
use actix_web::{get, post, put, web, Responder, delete};
use crate::modules::system::domain::{SysRole, SysUserRole, SysRoleEntity, SysRoleColumn, SysRoleDeptEntity, SysRoleDeptColumn, SysRoleDeptActiveModel};
use crate::modules::system::domain::bo::role::RoleDataScopeDTO;
use crate::modules::system::domain::bo::role::RoleIdsQueryDTO;
use sea_orm::{Set, EntityTrait, ColumnTrait, QueryFilter};
use crate::modules::system::dao::insert_batch;

#[get("/role/list")]
pub async fn page(arg: web::Query<RolePageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.page(&arg.0).await;
    return PageVO::from_result(&vo).resp_json();
}

#[get("/role/{role_id}")]
pub async fn detail(role_id: web::Path<String>) -> impl Responder {
    let role_id = role_id.into_inner();
    let role_vo = CONTEXT.sys_role_service.detail(&role_id).await;
    RespVO::from_result(&role_vo).resp_json()
}

#[post("/role")]
pub async fn add(arg: web::Json<RoleAddDTO>, current_user: CurrentUser) -> impl Responder {
    // 前端未勾选菜单时不传 menuIds，不能 unwrap 否则直接 panic（连接被关闭）
    let menu_ids = arg.0.menu_ids.clone().unwrap_or_default();
    let mut data = SysRole::from(arg.0);

    data.create_by = Some(current_user.id);
    let vo = CONTEXT.sys_role_service.add(data, menu_ids).await;
    return RespVO::from_result(&vo).resp_json();
}

#[put("/role")]
pub async fn update(arg: web::Json<RoleUpdateDTO>, current_user: CurrentUser) -> impl Responder {
    // 未传 menuIds 表示本次不改菜单授权，不能当成「清空授权」
    let menu_ids = arg.0.menu_ids.clone();
    let mut data = SysRole::from(arg.0);
    data.update_by = Some(current_user.id);
    let vo = CONTEXT.sys_role_service.update(data, menu_ids).await;
    return RespVO::from_result(&vo).resp_json();
}

#[delete("/role/{role_id}")]
pub async fn remove(role_id: web::Path<String>) -> impl Responder {
    let role_id = role_id.into_inner();
    let vo = CONTEXT.sys_role_service.remove(&role_id).await;
    return RespVO::from_result(&Ok(1)).resp_json();
}

//已分配此角色的用户
#[get("/role/authUser/allocatedList")]
pub async fn auth_user_list(arg: web::Query<RoleAuthUserPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.auth_user_list_page(&arg.0).await.unwrap();
    let total = CONTEXT.sys_role_service.auth_user_list_count(&arg.0).await.unwrap_or(0);
    let mut res = RespJson::success();
    res.insert("rows".to_string(), serde_json::json!(vo));
    res.insert("total".to_string(), serde_json::json!(total));
    res.resp_json()
}

//未分配此角色的用户
#[get("/role/authUser/unallocatedList")]
pub async fn unallocated_user_list(arg: web::Query<RoleAuthUserPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.unallocated_user_list_page(&arg.0).await.unwrap();
    let total = CONTEXT.sys_role_service.unallocated_user_list_count(&arg.0).await.unwrap_or(0);
    let mut res = RespJson::success();
    res.insert("rows".to_string(), serde_json::json!(vo));
    res.insert("total".to_string(), serde_json::json!(total));
    res.resp_json()
}

//取消对某个用户授权
#[put("/role/authUser/cancel")]
pub async fn cancel_user(arg: web::Json<UserRoleDTO>) -> impl Responder {
    let row = CONTEXT.sys_user_role_service.remove(&SysUserRole::from(arg.0)).await.unwrap();
    RespVO::<u64>::judge(row, "取消授权成功。".to_string(), "取消授权失败！".to_string()).resp_json()
}

//对多个用户进行授权
#[put("/role/authUser/selectAll")]
pub async fn auth_user_all(arg: web::Query<UsersRoleDTO>) -> impl Responder {
    let user_ids: Vec<String> = arg.user_ids.split(",").map(|u| u.to_string()).collect();
    let res = CONTEXT.sys_user_role_service.add_users_role(&arg.0.role_id, &user_ids).await;
    RespVO::<u64>::judge(res.unwrap(), "批量授权成功。".to_string(), "批量授权失败！".to_string()).resp_json()
}
//对多个用户进行授权
#[put("/role/authUser/cancelAll")]
pub async fn cancel_user_all(arg: web::Query<UsersRoleDTO>) -> impl Responder {
    let user_ids: Vec<String> = arg.user_ids.split(",").map(|u| u.to_string()).collect();
    let res = CONTEXT.sys_user_role_service.remove_users_role(&arg.0.role_id, &user_ids).await;
    RespVO::<u64>::judge(res.unwrap(), "批量取消授权成功。".to_string(), "批量取消授权失败！".to_string()).resp_json()
}

#[put("/role/changeStatus")]
pub async fn change_status(arg: web::Json<RoleUpdateDTO>, current_user: CurrentUser) -> impl Responder {

    //  roleService.checkRoleAllowed(role);  todo
    //         roleService.checkRoleDataScope(role.getRoleId()); todo
    //         role.setUpdateBy(getUsername());
    let mut data = SysRole::from(arg.0);
    data.update_by = Some(current_user.id);
    let res = CONTEXT.sys_role_service.update(data, None).await;
    return RespVO::from_result(&res).resp_json();
}

// 分配角色数据权限（对应Spring PUT /system/role/dataScope）
#[put("/role/dataScope")]
pub async fn data_scope(arg: web::Json<RoleDataScopeDTO>) -> impl Responder {
    let role_id = arg.role_id.clone().unwrap_or_default().parse::<i64>().unwrap_or(0);
    let data_scope = arg.data_scope.clone().unwrap_or_else(|| "1".to_string());
    let dept_ids = arg.dept_ids.clone().unwrap_or_default();
    let _ = SysRoleEntity::update_many()
        .col_expr(SysRoleColumn::DataScope, sea_orm::sea_query::Expr::value(data_scope))
        .filter(SysRoleColumn::RoleId.eq(role_id.clone()))
        .exec(crate::pool!())
        .await;
    let _ = SysRoleDeptEntity::delete_many()
        .filter(SysRoleDeptColumn::RoleId.eq(role_id.clone()))
        .exec(crate::pool!())
        .await;
    let mut rows = vec![];
    for dept_id in dept_ids {
        rows.push(SysRoleDeptActiveModel { role_id: Set(role_id), dept_id: Set(dept_id.parse::<i64>().unwrap_or(0)) });
    }
    if !rows.is_empty() {
        let _ = insert_batch(crate::pool!(), rows).await;
    }
    RespVO::<String>::from_success_info("更新成功！").resp_json()
}

// 角色下拉框（对应Spring GET /system/role/optionselect）
#[get("/role/optionselect")]
pub async fn optionselect(role_ids: web::Query<RoleIdsQueryDTO>) -> impl Responder {
    let ids: Vec<String> = role_ids.role_ids.clone().unwrap_or_default()
        .split(",").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let data = if ids.is_empty() {
        CONTEXT.sys_role_service.finds_all().await
    } else {
        CONTEXT.sys_role_service.finds_by_ids(&ids).await
    };
    RespVO::from_result(&data).resp_json()
}

// 查询角色部门树（对应Spring GET /system/role/deptTree/{roleId}）
#[get("/role/deptTree/{role_id}")]
pub async fn dept_tree(role_id: web::Path<String>, current_user: CurrentUser) -> impl Responder {
    let role_id = role_id.into_inner();
    let depts = CONTEXT.sys_dept_service.get_dept_tree(&current_user.name).await.unwrap_or_default();
    let checked_keys = SysRoleDeptEntity::find()
        .filter(SysRoleDeptColumn::RoleId.eq(role_id.clone()))
        .all(crate::pool!())
        .await
        .unwrap_or_default()
        .into_iter().map(|d| d.dept_id).collect::<Vec<i64>>();
    let mut res = RespJson::success();
    // RuoYi-Vue3（plus-ui）从 data 中取复合对象：data.depts / data.checkedKeys
    let data = serde_json::json!({
        "depts": depts,
        "checkedKeys": checked_keys
    });
    res.insert("data".to_string(), data);
    res.resp_json()
}

// 数据导出（对应Spring POST /role/export）——分批拉取，避免全量进内存
#[post("/role/export")]
pub async fn export(query: web::Query<RolePageDTO>) -> impl Responder {
    let base = query.0;
    let headers = ["角色编号", "角色名称", "权限字符", "显示顺序", "数据范围", "角色状态"];
    let fetch = |page_no: u64| -> BoxFuture<'static, Result<Vec<Vec<String>>, String>> {
        let mut dto = base.clone();
        dto.page_no = Some(page_no);
        dto.page_size = Some(500);
        Box::pin(async move {
            let pg = CONTEXT.sys_role_service.page(&dto).await.map_err(|e| e.to_string())?;
            Ok(pg.records.into_iter().map(|r| vec![
                r.role_id.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.role_name.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.role_key.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.role_sort.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.data_scope.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                r.status.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            ]).collect())
        })
    };
    match crate::util::excel::build_xlsx_streaming("导出数据", &headers, 500, fetch).await {
        Ok(bytes) => crate::util::excel::excel_response(bytes, "role.xlsx"),
        Err(e) => RespVO::<String>::from_error_info(500, &e).resp_json(),
    }
}
