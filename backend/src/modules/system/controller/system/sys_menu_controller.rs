use crate::modules::system::domain::bo::menu::{MenuAddDTO, MenuPageDTO, MenuUpdateDTO};
use crate::web_data::CurrentUser;
use crate::modules::system::domain::{SysMenu, SysMenuEntity, SysMenuColumn};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QuerySelect, TransactionTrait};
use std::collections::{HashMap, HashSet, VecDeque};
use crate::modules::system::domain::vo::{RespJson, RespVO};
use crate::modules::system::service::CONTEXT;
use actix_web::{get, post, put, delete, web, Responder, HttpRequest};
use crate::config::global_variables::ADMIN_NAME;

#[get("/menu/list")]
pub async fn query_menu(page: web::Query<MenuPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_menu_service.query_menu(&page.0).await;
    RespVO::from_result(&data).resp_json()
}


//菜单栏生成（对应Spring /system/menu/getRouters）
// 逻辑提取为内部函数，供 /menu/getRouters 与 RuoYi 标准前端顶层别名 /getRouters 复用
pub(crate) async fn build_routers(req: HttpRequest) -> impl Responder {
    let user_cache = CONTEXT.sys_user_service.get_user_cache(&req).await;
    let data = CONTEXT.sys_menu_service.get_routers(&user_cache.unwrap()).await;
    RespVO::from_result(&data).resp_json()
}

#[get("/menu/getRouters")]
pub async fn routers(req: HttpRequest) -> impl Responder {
    build_routers(req).await
}

// RuoYi 标准前端（ruoyi-vue3）顶层路径别名：/dev-api/getRouters
#[get("/getRouters")]
pub async fn routers_alias(req: HttpRequest) -> impl Responder {
    build_routers(req).await
}

#[get("/menu/{menu_id}")]
pub async fn detail(menu_id: web::Path<String>) -> impl Responder {
    let menu_id = menu_id.into_inner();
    let menu_vo = CONTEXT.sys_menu_service.detail(&menu_id).await;
    RespVO::from_result(&menu_vo).resp_json()
}

#[post("/menu")]
pub async fn add(arg: actix_web_validator::Json<MenuAddDTO>) -> impl Responder {
    let mut data = SysMenu::from(arg.0);
    // 菜单主键走「表内自增」：get_table_maxid 默认不加日期前缀，即 max(menu_id) + 1，
    // 与原实现保持同一量级（不用雪花 ID，避开前端路由/排序依赖旧 id 的问题）。
    // 注意：返回值必须真正赋给主键；旧代码查完就丢、又被雪花 ID 覆盖，等于白跑一次聚合查询。
    data.menu_id =
        crate::modules::system::service::ServiceContext::get_table_maxid("menu_id", "sys_menu")
            .await;

    if data.path.is_none() {
        data.path = Some("".to_string());
    }
    let data = CONTEXT.sys_menu_service.add(&data).await;
    CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

#[put("/menu")]
pub async fn update(current_user: CurrentUser, arg: web::Json<MenuUpdateDTO>) -> impl Responder {
    let mut menu = SysMenu::from(arg.0);
    menu.update_by = Some(current_user.id);

    let cnt = CONTEXT.sys_menu_service.update(&menu).await;

    CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&cnt).resp_json()
}

#[delete("/menu/{menu_id}")]
pub async fn remove(menu_id: web::Path<i64>) -> impl Responder {
    let menu_id = menu_id.into_inner();
    let data = CONTEXT.sys_menu_service
        .remove(&menu_id).await;
    CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

#[get("/menu/treeselect")]
pub async fn treeselect(req: HttpRequest) -> impl Responder {
    let user_cache = match CONTEXT.sys_user_service.get_user_cache(&req).await {
        Ok(u) => u,
        Err(e) => {
            return RespVO::<String>::from_error_info(401, &format!("Unauthorized for:{}", e)).resp_json();
        }
    };
    let menus = if user_cache.user_name == ADMIN_NAME {
        CONTEXT.sys_menu_service.all().await
    } else {
        CONTEXT.sys_menu_service.get_menu_list_by_user_id(user_cache.id).await
    };
    let menu_tree = CONTEXT.sys_menu_service.build_menu_tree(&menus.unwrap()).unwrap();
    let menu_select = CONTEXT.sys_menu_service.tree_select(menu_tree);
    RespVO::from_result(&menu_select).resp_json()
}

#[get("/menu/roleMenuTreeselect/{role_id}")]
pub async fn role_menu_treeselect(req: HttpRequest, role_id: web::Path<String>) -> impl Responder {
    let role_id = role_id.into_inner();

    let user_cache = match CONTEXT.sys_user_service.get_user_cache(&req).await {
        Ok(u) => u,
        Err(e) => {
            return RespVO::<String>::from_error_info(401, &format!("Unauthorized for:{}", e)).resp_json();
        }
    };
    let menus = if user_cache.user_name == ADMIN_NAME {
        CONTEXT.sys_menu_service.all().await
    } else {
        CONTEXT.sys_menu_service.get_menu_list_by_user_id(user_cache.id).await
    };
    let menu_tree = match CONTEXT.sys_menu_service.build_menu_tree(&menus.unwrap()) {
        Ok(t) => t,
        Err(e) => {
            return RespVO::<String>::from_error_info(500, &e.to_string()).resp_json();
        }
    };
    let menu_select = CONTEXT.sys_menu_service.tree_select(menu_tree);

    let checked_keys = CONTEXT.sys_role_menu_service.select_by_role_id(role_id.parse::<i64>().unwrap_or(0)).await.unwrap().into_iter().map(|m| m.menu_id).collect::<Vec<_>>();

    let mut res = RespJson::success();
    // RuoYi-Vue3（plus-ui）从 data 中取复合对象：data.menus / data.checkedKeys
    let data = serde_json::json!({
        "checkedKeys": checked_keys,
        "menus": menu_select.unwrap()
    });
    res.insert("data".to_string(), data);
    res.resp_json()
}


// 级联删除菜单（对应Spring DELETE /system/menu/cascade/{menuIds}）
#[delete("/menu/cascade/{menu_ids}")]
pub async fn cascade_delete(menu_ids: web::Path<String>) -> impl Responder {
    let menu_ids = menu_ids.into_inner();
    let roots: Vec<i64> = menu_ids
        .split(",")
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();
    if roots.is_empty() {
        return RespVO::from_result(&Ok(0u64)).resp_json();
    }

    // 1) 一次取出全表「menu_id -> parent_id」，在内存中 BFS 收集待删集合。
    //    原实现是「每节点一次 SELECT 查子节点」的 N+1，删一棵深树要打几十次库。
    let pairs: Vec<(Option<i64>, Option<i64>)> = SysMenuEntity::find()
        .select_only()
        .column(SysMenuColumn::MenuId)
        .column(SysMenuColumn::ParentId)
        .into_tuple()
        .all(crate::pool!())
        .await
        .unwrap_or_default();

    let mut children_of: HashMap<i64, Vec<i64>> = HashMap::with_capacity(pairs.len());
    for (mid, pid) in pairs {
        if let (Some(m), Some(p)) = (mid, pid) {
            children_of.entry(p).or_default().push(m);
        }
    }

    let mut to_delete: Vec<i64> = Vec::new();
    let mut seen: HashSet<i64> = HashSet::new();
    let mut queue: VecDeque<i64> = roots.into_iter().collect();
    while let Some(id) = queue.pop_front() {
        if !seen.insert(id) {
            continue; // 多个根节点可能共享子孙，去重
        }
        to_delete.push(id);
        if let Some(kids) = children_of.get(&id) {
            queue.extend(kids.iter().copied());
        }
    }

    // 2) 批量删除：菜单、角色-菜单关联必须在**同一事务**内完成。
    //    否则「菜单删成功、清关联失败」会留下指向已删 menu_id 的孤儿 role_menu 行，
    //    而旧实现还用 `let _ =` 吞掉关联清理的错误，故障完全不可见。
    let txn = match crate::pool!().begin().await {
        Ok(t) => t,
        Err(e) => {
            log::error!("[menu cascade] begin txn failed: {e}");
            return RespVO::from_error_result(500, &Err::<u64, _>(crate::error::Error::from(e)))
                .resp_json();
        }
    };

    let affected = match CONTEXT
        .sys_menu_service
        .remove_batch_in_txn(&txn, &to_delete)
        .await
    {
        Ok(n) => n,
        Err(e) => {
            // txn 在此被 drop，sea-orm 自动 rollback
            log::error!("[menu cascade] delete menus failed, rolled back: {e}");
            return RespVO::from_error_result(500, &Err::<u64, _>(e)).resp_json();
        }
    };

    if let Err(e) = CONTEXT
        .sys_role_menu_service
        .remove_by_menu_ids_in_txn(&txn, &to_delete)
        .await
    {
        log::error!("[menu cascade] clear role_menu failed, rolled back: {e}");
        return RespVO::from_error_result(500, &Err::<u64, _>(e)).resp_json();
    }

    if let Err(e) = txn.commit().await {
        log::error!("[menu cascade] commit failed: {e}");
        return RespVO::from_error_result(500, &Err::<u64, _>(crate::error::Error::from(e)))
            .resp_json();
    }

    // 3) 统一刷新一次菜单缓存（务必在提交成功之后，避免回滚窗口读到脏缓存）
    let _ = CONTEXT.sys_menu_service.update_cache().await;
    RespVO::from_result(&Ok(affected)).resp_json()
}
