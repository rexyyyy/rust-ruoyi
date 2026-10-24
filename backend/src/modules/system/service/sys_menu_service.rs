use std::collections::{BTreeMap, HashMap};
use crate::config::global_variables::{CHAR_FALSE, INNER_LINK, LAYOUT, PARENT_VIEW, TYPE_DIR, TYPE_MENU, ADMIN_NAME};
use crate::modules::system::domain::bo::menu::MenuPageDTO;

use crate::modules::system::domain::{SysMenu, SysMenuEntity, SysMenuColumn};
use crate::modules::system::domain::vo::{MenuTreeSelectVO, MetaVO, RouterVO, SysMenuVO, UserCache};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use crate::util::string::capitalize;
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::*;
use crate::modules::system::dao::*;
// 缓存直接存转换后的 SysMenuVO（key 带 _vo 与旧格式区分，旧 key 数据会自然过期）
const RES_MENU_KEY: &'static str = "sys_menu:all_vo";

/// Menu service
pub struct SysMenuService {}

impl SysMenuService {
    pub async fn query_menu(&self, query: &MenuPageDTO) -> Result<Vec<SysMenuVO>> {
        let mut select = SysMenuEntity::find();
        if let Some(name) = &query.menu_name {
            if !name.is_empty() {
                select = select.filter(SysMenuColumn::MenuName.contains(name.as_str()));
            }
        }
        if let Some(status) = &query.status {
            select = select.filter(SysMenuColumn::Status.eq(status.to_string()));
        }
        select = select.order_by_asc(SysMenuColumn::OrderNum);
        let res: Vec<SysMenuVO> = select
            .all(pool!())
            .await?
            .into_iter()
            .map(|m| SysMenuVO::from(m))
            .collect();
        Ok(res)
    }
    /// Find the menu array
    pub async fn all(&self) -> Result<Vec<SysMenuVO>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysMenu>>>(RES_MENU_KEY)
            .await;
        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        if CONTEXT.config.debug {
            log::info!("[ruoyi_rust] get from redis:{}", RES_MENU_KEY);
        }
        let mut arr = vec![];
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        return Ok(arr);
    }

    ///menu details
    pub async fn detail(&self, menu_id: &str) -> Result<SysMenuVO> {
        let menuid: i64 = menu_id.parse().unwrap_or(0);
        // one() 走 LIMIT 1，避免 `all()` 先物化整个结果集再取首行
        let menu = SysMenuEntity::find()
            .filter(SysMenuColumn::MenuId.eq(menuid))
            .one(pool!())
            .await?
            .ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", menu_id)))?;
        let menu_vo = SysMenuVO::from(menu);
        return Ok(menu_vo);
    }
    pub async fn add(&self, arg: &SysMenu) -> Result<u64> {
        let result = insert_model(pool!(), arg.clone()).await?;
        self.update_cache().await?;
        return Ok(result);
    }

    pub async fn update(&self, arg: &SysMenu) -> Result<u64> {
        let result = update_model(pool!(), arg.clone()).await?;
        self.update_cache().await?;
        return Ok(result);
    }

    pub async fn remove(&self, id: &i64) -> Result<u64> {
        let num = SysMenuEntity::delete_many()
            .filter(SysMenuColumn::MenuId.eq(*id))
            .exec(pool!())
            .await?
            .rows_affected;
        // TODO  CONTEXT.sys_trash_service.add("sys_menu", &trash).await?;

        self.update_cache().await?;
        return Ok(num);
    }

    /// 批量删除菜单（**不刷新**菜单缓存）。
    ///
    /// 与 [`Self::remove`] 的差别就是不动缓存：级联删除会一次删掉几十上百个节点，
    /// 若逐个走 `remove`，每个都会触发一次「全表 SELECT + Redis SET」的缓存刷新，
    /// 形成严重的写放大。批量场景应由调用方在末尾统一 `update_cache()` 一次。
    pub async fn remove_batch(&self, ids: &[i64]) -> Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        Ok(SysMenuEntity::delete_many()
            .filter(SysMenuColumn::MenuId.is_in(ids.to_vec()))
            .exec(pool!())
            .await?
            .rows_affected)
    }

    pub async fn get_menu_list_by_user_id(&self, user_id: i64) -> Result<Vec<SysMenuVO>> {
        let sql = "
      select distinct m.menu_id, m.parent_id, m.menu_name, m.path, m.component, m.query_param, m.visible, m.status, coalesce(m.perms,'') as perms, m.is_frame, m.is_cache, m.menu_type, m.icon, m.order_num, m.create_time, m.create_by, m.update_by, m.update_time, m.remark
		from sys_menu m
		left join sys_role_menu rm on m.menu_id = rm.menu_id
		left join sys_user_role ur on rm.role_id = ur.role_id
		left join sys_role ro on ur.role_id = ro.role_id
		where ur.user_id = $1
		order by m.parent_id, m.order_num
       ";
        let values: Vec<sea_orm::Value> = vec![Value::from(user_id)];
        let rows = pool!()
            .query_all(crate::db::raw_stmt( sql, values))
            .await?;
        let data: Vec<SysMenu> = rows
            .iter()
            .filter_map(|r| SysMenu::from_query_result(r, "").ok())
            .collect();
        Ok(data.into_iter().map(|m| m.into()).collect())
    }

    pub async fn update_cache(&self) -> Result<Vec<SysMenuVO>> {
        let all = SysMenuEntity::find()
            .order_by_asc(SysMenuColumn::OrderNum)
            .all(pool!())
            .await?;
        // 只转换一次，缓存 VO 形态
        let v: Vec<SysMenuVO> = all.into_iter().map(|x| x.into()).collect();
        CONTEXT.cache_service.set_json(RES_MENU_KEY, &v).await?;
        Ok(v)
    }

    pub async fn finds_all_map(&self) -> Result<BTreeMap<i64, SysMenuVO>> {
        let all = self.all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.menu_id.unwrap_or_default(), x);
        }
        return Ok(result);
    }

    pub fn finds_menu(
        &self,
        ids: &Vec<i64>,
        all_menus: &BTreeMap<i64, SysMenuVO>,
    ) -> Vec<SysMenuVO> {
        // 由「遍历全部菜单 × 遍历 ids」的 O(n*m) 改为按 id 直接查表 O(ids·log n)
        ids.iter().filter_map(|id| all_menus.get(id).cloned()).collect()
    }


    //变成id 和label
    pub fn tree_select(&self, menus: Vec<SysMenuVO>) -> Result<Vec<MenuTreeSelectVO>> {
        let mut d = vec![];
        for menu in menus {
            let mut t = MenuTreeSelectVO {
                id: menu.menu_id,
                label: menu.menu_name,
                children: None,
            };
            let c = menu.children.unwrap();
            if !c.is_empty() {
                t.children = Some(self.tree_select(c)?);
            }
            d.push(t);
        }
        Ok(d)
    }
    ///An menus array with a hierarchy
    ///
    /// 由「对每个节点全表扫描其子节点」的 O(n²) 改为一次建索引 + 递归挂载的 O(n)：
    /// 先按 parent_id 归组，再自顶向下挂 children。
    pub fn build_menu_tree(&self, all_menus: &Vec<SysMenuVO>) -> Result<Vec<SysMenuVO>> {
        // 一次线性扫描建立 parent_id -> children 索引
        let mut children_map: HashMap<i64, Vec<SysMenuVO>> = HashMap::new();
        for item in all_menus {
            if let Some(pid) = item.parent_id {
                if pid != 0 {
                    children_map.entry(pid).or_default().push(item.clone());
                }
            }
        }
        // find tops（parent_id == 0）
        let mut tops: Vec<SysMenuVO> = all_menus
            .iter()
            .filter(|m| m.is_parent())
            .cloned()
            .collect();
        tops.sort_by(|a, b| a.order_num.cmp(&b.order_num));
        for item in tops.iter_mut() {
            Self::attach_children(item, &children_map);
        }
        Ok(tops)
    }

    /// 递归挂载子节点（每个节点只访问自己的直接子节点，整体 O(n log n)，排序主导）
    fn attach_children(arg: &mut SysMenuVO, children_map: &HashMap<i64, Vec<SysMenuVO>>) {
        if let Some(mid) = arg.menu_id {
            if let Some(kids) = children_map.get(&mid) {
                let mut kids = kids.clone();
                kids.sort_by(|a, b| a.order_num.cmp(&b.order_num));
                for k in kids.iter_mut() {
                    Self::attach_children(k, children_map);
                }
                arg.children = Some(kids);
            }
        }
    }

    ///生成菜单
    pub async fn get_routers(&self, user_cache: &UserCache) -> Result<Vec<RouterVO>> {
        let all_menus = self.all().await?;
        let filtered_menus = if user_cache.user_name == ADMIN_NAME {
            all_menus
        } else {
            // 原实现是「遍历全部菜单 × 遍历用户 menu_ids」的双层扫描 O(n*m)，
            // 每个命中还要 clone 一份。改为先把 menu_ids 建成哈希集合，
            // 再对菜单做单次线性过滤，复杂度降为 O(n+m)。
            let owned: std::collections::HashSet<i64> =
                user_cache.menu_ids.iter().copied().collect();
            all_menus
                .into_iter()
                .filter(|v| v.menu_id.map(|id| owned.contains(&id)).unwrap_or(false))
                .collect()
        };

        let menu_tree = self.build_menu_tree(&filtered_menus)?;
        Ok(self.build_routers(&menu_tree))
    }

    fn build_routers(&self, menus: &Vec<SysMenuVO>) -> Vec<RouterVO> {
        let mut routers = vec![];
        for menu in menus {
            let menu_type = menu.menu_type.clone().unwrap_or_default();
            if menu_type != TYPE_DIR.to_string() && menu_type != TYPE_MENU.to_string() { continue; }
            let mut router = RouterVO {
                name: Some(self.get_route_name(&menu)),
                path: Some(self.get_router_path(&menu)),
                hidden: Some(menu.visible.clone().unwrap() == CHAR_FALSE.to_string()),
                redirect: None,
                component: Some(self.get_component(&menu)),
                query: menu.query.clone(),
                always_show: None,
                meta: MetaVO {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    no_cache: Some(menu.is_cache.clone().unwrap() == CHAR_FALSE.to_string()),
                    link: None,
                }.into(),
                children: vec![],
            };
            let c_menus = menu.children.clone().unwrap();
            if c_menus.len() > 0 && menu.menu_type.clone().unwrap() == TYPE_DIR.to_string() {
                router.always_show = Some(true);
                router.redirect = Some("noRedirect".to_string());
                router.children = self.build_routers(&c_menus);
            } else if menu.is_menu_frame() {
                let mut children_list = vec![];
                let children = RouterVO {
                    name: Some(format!("{}{}", capitalize(&menu.path.clone().unwrap()), menu.menu_id.unwrap_or_default())),//大写+menuId保证唯一
                    path: menu.path.clone(),
                    hidden: Some(false),
                    redirect: None,
                    component: menu.component.clone(),
                    query: menu.query.clone(),
                    always_show: None,
                    meta: MetaVO {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        no_cache: Some(menu.is_cache.clone().unwrap() == CHAR_FALSE.to_string()),
                        link: menu.path.clone(),
                    }.into(),
                    children: vec![],
                };
                children_list.push(children);
                router.children = children_list;
            } else if menu.is_parent() && menu.is_inner_link() {
                router.meta = MetaVO {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    no_cache: Some(false),
                    link: None,
                }.into();
                router.path = Some("/".to_string());
                let mut children_list = vec![];
                let children = RouterVO {
                    name: Some(format!("{}{}", capitalize(&menu.path.clone().unwrap()), menu.menu_id.unwrap_or_default())),//大写+menuId保证唯一
                    path: menu.path.clone(),
                    hidden: Some(false),
                    redirect: None,
                    component: Some(INNER_LINK.to_string()),
                    query: menu.query.clone(),
                    always_show: None,
                    meta: MetaVO {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        no_cache: Some(true),
                        link: menu.path.clone(),
                    }.into(),
                    children: vec![],
                };
                children_list.push(children);
                router.children = children_list;
            }
            routers.push(router);
        }
        routers
    }
    /**
     * 获取路由名称
     *
     * @param menu 菜单信息
     * @return 路由名称
     */
    fn get_route_name(&self, menu: &SysMenuVO) -> String {
        // 非外链并且是一级目录（类型为目录）
        let base = if menu.is_menu_frame() {
            "".to_string()
        } else {
            capitalize(menu.path.as_deref().unwrap_or_default())
        };
        // 追加 menuId 保证路由 name 全局唯一（镜像 Java: getRouteName() + menuId），
        // 否则不同菜单若 path 片段相同会产生重名，vue-router addRoute 会互相顶掉导致 404。
        format!("{}{}", base, menu.menu_id.unwrap_or_default())
    }
    /**
     * 获取路由地址
     *
     * @param menu 菜单信息
     * @return 路由地址
     */
    fn get_router_path(&self, menu: &SysMenuVO) -> String {
        let mut router_path = menu.path.clone().unwrap();
        // 内链打开外网方式
        if !menu.is_parent() && menu.is_inner_link() {
            // router_path = innerLinkReplaceEach(router_path);
        }
        // 非外链并且是一级目录（类型为目录）

        if menu.is_parent() && menu.menu_type.clone().unwrap() == TYPE_DIR.to_string()
            && menu.is_frame.clone().unwrap() == CHAR_FALSE.to_string() {
            router_path = format!("/{}", router_path);
        }
        // 非外链并且是一级目录（类型为菜单）
        else if menu.is_menu_frame() {
            router_path = "/".to_string();
        }
        router_path
    }
    /**
     * 获取组件信息
     *
     * @param menu 菜单信息
     * @return 组件信息
     */
    fn get_component(&self, menu: &SysMenuVO) -> String {
        let old_component = menu.component.as_deref().unwrap_or_default();
        let mut component = LAYOUT;
        if old_component.len() > 0 && !menu.is_menu_frame() {
            component = menu.component.as_ref().unwrap();
        } else if old_component.len() == 0 && !menu.is_parent() && menu.is_inner_link()
        {
            component = INNER_LINK;
        } else if old_component.len() == 0 && menu.is_parent_view()
        {
            component = PARENT_VIEW;
        }
        component.to_string()
    }
}
