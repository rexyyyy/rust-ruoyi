//! 设计器扩展点实现：把 RuoYi 的部门 / 角色 / 用户 / 流程分类喂给 warm-flow 设计器。
//!
//! warm-flow 的 `UiExtension` 里 `categories()` / `form_paths()` 是**同步**函数，
//! 而这两类数据都在数据库里。处理方式与 [`super::permission`] 一致：
//! 内存快照 + 分类变更后主动刷新，避免在 async 上下文里 block_on。
//!
//! 其余 `handler_select` / `handler_feedback` 是 async，直接查库。

use std::sync::Arc;

use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use sea_orm::{ConnectionTrait, FromQueryResult, Value};
use warm_flow_core::dto::{DefJson, Tree};
use warm_flow_web::vo::{Dict, HandlerAuth, HandlerFeedBackVo, HandlerQuery, HandlerSelectVo, NodeExt};
use warm_flow_web::vo::PageVo;
use warm_flow_web::UiExtension;

use crate::db::{raw_stmt, raw_stmt0};

/// 办理人选择页签：部门 / 角色 / 用户
pub const HANDLER_TYPE_DEPT: &str = "dept";
pub const HANDLER_TYPE_ROLE: &str = "role";
pub const HANDLER_TYPE_USER: &str = "user";

/// 外挂表单（设计器「表单路径」下拉），对应 plus-ui 的 `views/**` 路由。
/// 新增业务表单时在这里追加即可，无需改设计器。
const FORM_PATHS: [(&str, &str); 1] = [("/workflow/leaveEdit/index", "请假申请表单")];

pub struct RuoYiUiExtension {
    /// 流程分类树快照（扁平 -> 树在写入前完成）
    /// 注意不能叫 `categories`：与 `UiExtension::categories()` 同名时方法调用会被字段遮蔽
    category_tree: RwLock<Vec<Tree>>,
}

static UI: OnceCell<Arc<RuoYiUiExtension>> = OnceCell::new();

/// 取全局扩展点实例（未初始化时惰性建一个空实例，保证设计器至少能打开）
pub fn ui() -> &'static Arc<RuoYiUiExtension> {
    UI.get_or_init(|| Arc::new(RuoYiUiExtension::new()))
}

/// 初始化：加载分类快照并登记到全局
pub async fn init() -> Arc<RuoYiUiExtension> {
    let inst = UI.get_or_init(|| Arc::new(RuoYiUiExtension::new()));
    if let Err(e) = inst.reload_categories().await {
        log::warn!("[workflow] 流程分类快照加载失败: {}", e);
    }
    inst.clone()
}

#[derive(Debug, FromQueryResult)]
struct CategoryRow {
    category_id: i64,
    parent_id: i64,
    category_name: String,
}

#[derive(Debug, FromQueryResult)]
struct DeptRow {
    dept_id: i64,
    dept_name: String,
}

#[derive(Debug, FromQueryResult)]
struct RoleRow {
    role_id: i64,
    role_name: String,
}

#[derive(Debug, FromQueryResult)]
struct UserRow {
    user_id: i64,
    user_name: String,
    nick_name: Option<String>,
}

impl Default for RuoYiUiExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl RuoYiUiExtension {
    pub fn new() -> Self {
        Self {
            category_tree: RwLock::new(Vec::new()),
        }
    }

    /// 全量刷新流程分类快照。分类增删改后由 service 主动调用。
    pub async fn reload_categories(&self) -> Result<(), String> {
        let db = crate::db::db().ok_or_else(|| "数据库未初始化".to_string())?;
        let rows = db
            .query_all(raw_stmt0(
                "SELECT category_id AS category_id, parent_id AS parent_id, category_name AS category_name \
                 FROM flow_category WHERE del_flag = '0' OR del_flag IS NULL \
                 ORDER BY parent_id, order_num, category_id",
            ))
            .await
            .map_err(|e| format!("查询流程分类失败: {e}"))?;

        let flat: Vec<Tree> = rows
            .iter()
            .filter_map(|r| CategoryRow::from_query_result(r, "").ok())
            .map(|r| Tree {
                id: Some(r.category_id.to_string()),
                name: Some(r.category_name.clone()),
                parent_id: Some(r.parent_id.to_string()),
                label: Some(r.category_name),
                value: Some(r.category_id.to_string()),
                children: Vec::new(),
            })
            .collect();

        *self.category_tree.write() = Tree::build_tree(flat);
        Ok(())
    }
}

#[async_trait::async_trait]
impl UiExtension for RuoYiUiExtension {
    fn framework(&self) -> String {
        "ruoyi-rust".to_string()
    }

    fn handler_types(&self) -> Vec<String> {
        vec![
            HANDLER_TYPE_DEPT.to_string(),
            HANDLER_TYPE_ROLE.to_string(),
            HANDLER_TYPE_USER.to_string(),
        ]
    }

    fn categories(&self) -> Vec<Tree> {
        self.category_tree.read().clone()
    }

    fn form_paths(&self) -> Vec<Tree> {
        FORM_PATHS.iter().map(|(path, name)| Tree::leaf(*path, *name)).collect()
    }

    fn handler_dict(&self) -> Vec<Dict> {
        vec![
            Dict::new("默认表达式", "${handler}"),
            Dict::new("发起人", "${initiator}"),
            Dict::new("发起人部门", "${initiatorDeptId}"),
        ]
    }

    /// 办理人选择面板。`handler_type` 由前端页签决定，缺省按用户查询。
    async fn handler_select(&self, query: &HandlerQuery) -> HandlerSelectVo {
        let kind = query
            .handler_type
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(HANDLER_TYPE_USER);
        let mut rows = match kind {
            HANDLER_TYPE_DEPT => self.select_depts(query).await,
            HANDLER_TYPE_ROLE => self.select_roles(query).await,
            _ => self.select_users(query).await,
        };
        let total = rows.len() as u64;
        // 内存分页：部门/角色量级很小，用户已用 SQL 的 limit 兜住上限
        let offset = ((query.page_num() - 1) * query.page_size()) as usize;
        let rows: Vec<HandlerAuth> = rows
            .drain(..)
            .skip(offset)
            .take(query.page_size() as usize)
            .collect();
        HandlerSelectVo {
            handler_auths: PageVo::new(rows, total),
            tree_selections: Vec::new(),
        }
    }

    async fn handler_feedback(&self, storage_ids: &[String]) -> Vec<HandlerFeedBackVo> {
        let mut out = Vec::with_capacity(storage_ids.len());
        for id in storage_ids {
            let key = id.trim();
            if key.is_empty() {
                continue;
            }
            let name = self.resolve_name(key).await;
            out.push(HandlerFeedBackVo {
                storage_id: Some(key.to_string()),
                handler_name: name,
            });
        }
        out
    }

    /// 节点扩展属性：这里不额外扩展，按钮权限由节点 `ext` 自行携带
    fn node_ext(&self) -> Vec<NodeExt> {
        Vec::new()
    }

    /// 流程图顶部显示「流程名称 / 当前处理人」
    fn chart_ext(&self, def_json: &mut DefJson) {
        if def_json.top_text.is_none() {
            let name = def_json.flow_name.clone().unwrap_or_default();
            let node = def_json
                .instance
                .as_ref()
                .and_then(|ins| ins.node_name.clone())
                .unwrap_or_default();
            if !name.is_empty() {
                def_json.top_text = Some(if node.is_empty() {
                    name
                } else {
                    format!("{name} · 当前节点：{node}")
                });
            }
        }
    }
}

impl RuoYiUiExtension {
    async fn select_depts(&self, query: &HandlerQuery) -> Vec<HandlerAuth> {
        let kw = query
            .handler_name
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| format!("%{s}%"));
        let sql = match kw {
            Some(_) => {
                "SELECT dept_id, dept_name FROM sys_dept \
                 WHERE (del_flag = '0' OR del_flag IS NULL) AND dept_name LIKE $1 \
                 ORDER BY parent_id, order_num LIMIT 500"
            }
            None => {
                "SELECT dept_id, dept_name FROM sys_dept \
                 WHERE del_flag = '0' OR del_flag IS NULL \
                 ORDER BY parent_id, order_num LIMIT 500"
            }
        };
        let stmt = match &kw {
            Some(k) => raw_stmt(sql, vec![Value::from(k.clone())]),
            None => raw_stmt0(sql),
        };
        let rows = self.query_rows(stmt).await;
        let mut out = Vec::new();
        for row in rows {
            if let Ok(r) = DeptRow::from_query_result(&row, "") {
                out.push(HandlerAuth {
                    storage_id: Some(format!("{}:{}", HANDLER_TYPE_DEPT, r.dept_id)),
                    handler_code: Some(r.dept_id.to_string()),
                    handler_name: Some(r.dept_name),
                    group_name: Some("部门".to_string()),
                    create_time: None,
                });
            }
        }
        out
    }

    async fn select_roles(&self, query: &HandlerQuery) -> Vec<HandlerAuth> {
        let kw = query
            .handler_name
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| format!("%{s}%"));
        let sql = match kw {
            Some(_) => {
                "SELECT role_id, role_name FROM sys_role \
                 WHERE (del_flag = '0' OR del_flag IS NULL) AND role_name LIKE $1 \
                 ORDER BY role_sort LIMIT 500"
            }
            None => {
                "SELECT role_id, role_name FROM sys_role \
                 WHERE del_flag = '0' OR del_flag IS NULL \
                 ORDER BY role_sort LIMIT 500"
            }
        };
        let stmt = match &kw {
            Some(k) => raw_stmt(sql, vec![Value::from(k.clone())]),
            None => raw_stmt0(sql),
        };
        let rows = self.query_rows(stmt).await;
        let mut out = Vec::new();
        for row in rows {
            if let Ok(r) = RoleRow::from_query_result(&row, "") {
                out.push(HandlerAuth {
                    storage_id: Some(format!("{}:{}", HANDLER_TYPE_ROLE, r.role_id)),
                    handler_code: Some(r.role_id.to_string()),
                    handler_name: Some(r.role_name),
                    group_name: Some("角色".to_string()),
                    create_time: None,
                });
            }
        }
        out
    }

    async fn select_users(&self, query: &HandlerQuery) -> Vec<HandlerAuth> {
        // 用户量可能很大，SQL 侧就做过滤与截断
        let kw = query
            .handler_name
            .as_deref()
            .or(query.handler_code.as_deref())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| format!("%{s}%"));
        let sql = match kw {
            Some(_) => {
                "SELECT user_id, user_name, nick_name FROM sys_user \
                 WHERE (del_flag = '0' OR del_flag IS NULL) AND status = '0' \
                   AND (user_name LIKE $1 OR nick_name LIKE $1) \
                 ORDER BY user_id LIMIT 200"
            }
            None => {
                "SELECT user_id, user_name, nick_name FROM sys_user \
                 WHERE (del_flag = '0' OR del_flag IS NULL) AND status = '0' \
                 ORDER BY user_id LIMIT 200"
            }
        };
        let stmt = match &kw {
            Some(k) => raw_stmt(sql, vec![Value::from(k.clone())]),
            None => raw_stmt0(sql),
        };
        let rows = self.query_rows(stmt).await;
        let mut out = Vec::new();
        for row in rows {
            if let Ok(r) = UserRow::from_query_result(&row, "") {
                out.push(HandlerAuth {
                    storage_id: Some(format!("{}:{}", HANDLER_TYPE_USER, r.user_id)),
                    handler_code: Some(r.user_name.clone()),
                    handler_name: Some(match r.nick_name {
                        Some(n) if !n.is_empty() => format!("{n}（{}）", r.user_name),
                        _ => r.user_name,
                    }),
                    group_name: Some("用户".to_string()),
                    create_time: None,
                });
            }
        }
        out
    }

    async fn resolve_name(&self, key: &str) -> Option<String> {
        let (kind, id) = key.split_once(':').unwrap_or((HANDLER_TYPE_USER, key));
        let id: i64 = id.trim().parse().ok()?;
        let (sql, col) = match kind {
            HANDLER_TYPE_DEPT => ("SELECT dept_name AS name FROM sys_dept WHERE dept_id = $1", "name"),
            HANDLER_TYPE_ROLE => ("SELECT role_name AS name FROM sys_role WHERE role_id = $1", "name"),
            _ => ("SELECT user_name AS name FROM sys_user WHERE user_id = $1", "name"),
        };
        let db = crate::db::db()?;
        let row = db
            .query_one(raw_stmt(sql, vec![Value::from(id)]))
            .await
            .ok()??;
        row.try_get::<String>("", col).ok()
    }

    async fn query_rows(&self, stmt: sea_orm::Statement) -> Vec<sea_orm::QueryResult> {
        let Some(db) = crate::db::db() else {
            return Vec::new();
        };
        match db.query_all(stmt).await {
            Ok(rows) => rows,
            Err(e) => {
                log::warn!("[workflow] 设计器办理人查询失败: {}", e);
                Vec::new()
            }
        }
    }
}
