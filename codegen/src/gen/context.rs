//! 生成上下文：把「表元数据 + 用户配置」压平成一个可直接喂给模板的对象。
//!
//! 旧版本把这套推导散在 `backcode.rs` 的 `writeln!` 里（近 1200 行），
//! 改一个字段要翻遍全文。现在统一在这里算好，模板只负责排版。

use std::collections::HashMap;

use chrono::Local;
use serde::Serialize;

use crate::meta::typemap::{
    default_is_edit, default_is_insert, default_is_list, default_is_query, default_is_required,
    html_type_of, is_common_column, query_type_of, rust_type_of,
};
use crate::meta::TableMeta;
use crate::naming::{business_name, capitalize_first, to_camel, to_pascal};
use crate::util::ts::ts_type_of;

/// 单列的可覆盖项。`None` 表示沿用启发式默认值。
#[derive(Clone, Debug, Default)]
pub struct GenColumnOverride {
    pub is_insert: Option<bool>,
    pub is_edit: Option<bool>,
    pub is_list: Option<bool>,
    pub is_query: Option<bool>,
    pub is_required: Option<bool>,
    pub html_type: Option<String>,
    pub query_type: Option<String>,
    pub dict_type: Option<String>,
}

/// 一张表的生成参数
#[derive(Clone, Debug)]
pub struct GenOptions {
    /// 业务模块名（也是一级菜单 path / 权限前缀），如 `demo`
    pub module_name: String,
    /// 业务名（二级路径、路由名），如 `course`；为空则按表名推导
    pub business_name: Option<String>,
    /// 功能中文名，如 `课程`；为空则取表注释
    pub function_name: Option<String>,
    /// 作者
    pub function_author: String,
    /// 挂载到哪个菜单下
    pub parent_menu_id: i64,
    /// 逐列覆盖
    pub columns: HashMap<String, GenColumnOverride>,
}

impl Default for GenOptions {
    fn default() -> Self {
        GenOptions {
            module_name: "demo".to_string(),
            business_name: None,
            function_name: None,
            function_author: "ruoyi-rust".to_string(),
            parent_menu_id: 0,
            columns: HashMap::new(),
        }
    }
}

/// actix-web 路由路径。
///
/// 之所以在 Rust 侧拼好而不是在模板里拼：路由里的 `{id}` 花括号与 Tera 的
/// `{{ }}` 表达式定界符同形，模板里只能写成 `{{ "{" }}{{ pk.column_name }}{{ "}" }}`
/// 这种读都读不通的形式。预生成后模板里直接写 `{{ routes.detail }}` 即可。
#[derive(Clone, Debug, Serialize)]
pub struct RouteCtx {
    pub list: String,
    pub detail: String,
    pub add: String,
    pub update: String,
    pub remove: String,
    pub export: String,
}

fn routes_of(business: &str, pk_column: &str) -> RouteCtx {
    RouteCtx {
        list: format!("/{business}/list"),
        detail: format!("/{business}/{{{pk_column}}}"),
        add: format!("/{business}"),
        update: format!("/{business}"),
        remove: format!("/{business}/{{{pk_column}}}"),
        export: format!("/{business}/export"),
    }
}

/// 主键信息
#[derive(Clone, Debug, Serialize)]
pub struct PkCtx {
    pub column_name: String,
    /// sea-orm `Column` 枚举变体名，如 `CourseId`
    pub cap_field: String,
    /// 前端属性名
    pub prop: String,
    pub rust_type: String,
    /// 主键是否字符串型（决定 web::Path 取 i64 还是 String）
    pub is_string: bool,
}

/// 渲染上下文里的一列
#[derive(Clone, Debug, Serialize)]
pub struct ColCtx {
    pub column_name: String,
    /// Rust / 实体字段名（snake_case，与列名一致）
    pub field: String,
    /// sea-orm Column 变体名
    pub cap_field: String,
    /// 前端 / JSON 属性名（camelCase）
    pub prop: String,
    pub comment: String,
    pub rust_type: String,
    pub ts_type: String,
    pub is_pk: bool,
    pub is_increment: bool,
    pub nullable: bool,
    pub is_insert: bool,
    pub is_edit: bool,
    pub is_list: bool,
    pub is_query: bool,
    pub is_required: bool,
    pub query_type: String,
    pub html_type: String,
    pub dict_type: String,
    pub max_length: i64,
    pub is_numeric: bool,
    pub is_string: bool,
    pub is_datetime: bool,
    pub is_bool: bool,
    /// 审计列（create_by/create_time/...），表单里不出现
    pub is_common: bool,
    /// 生成「当前时间」的 Rust 表达式（仅时间列有意义，其它列为空串）
    pub now_expr: String,
}

/// 模板渲染上下文
#[derive(Clone, Debug, Serialize)]
pub struct GenCtx {
    pub table_name: String,
    pub table_comment: String,
    pub module_name: String,
    pub business_name: String,
    pub function_name: String,
    pub function_author: String,
    /// 类名（PascalCase），如 `DemoCourse`
    pub class_name: String,
    /// 服务 / 文件前缀，如 `course`
    pub biz_file: String,
    pub pk: PkCtx,
    pub columns: Vec<ColCtx>,
    /// 形如 `/demo/course`
    pub api_path: String,
    /// 权限前缀，形如 `demo:course`
    pub perm: String,
    /// 后端 Rust 模块内层目录名（与 module_name 一致）
    pub rust_module: String,
    /// 用到的字典类型（去重），供 Vue 模板生成 `useDict(...)`
    pub dict_types: Vec<String>,
    pub has_create_time: bool,
    pub has_update_time: bool,
    pub parent_menu_id: i64,
    pub generated_at: String,
    /// actix-web 路由路径（预生成，见 [`RouteCtx`]）
    pub routes: RouteCtx,
    /// 新建记录的 ID 初值表达式（雪花 ID / uuid）
    pub pk_new_expr: String,
    /// 把路径上的 `&str` 主键值转成实体主键类型的表达式（模板里绑到局部变量 `id`）
    pub pk_parse_expr: String,
    /// `create_by` 的赋值表达式（空串表示该表没有这一列，模板里就不生成这行）
    pub create_by_expr: String,
    /// `update_by` 的赋值表达式
    pub update_by_expr: String,
    // ---------- 前端模板派生量（模板里算会难以阅读，故在这里算好）----------
    /// Vue 列表变量名，如 `courseList`
    pub list_var: String,
    /// Vue 表单 ref 名，如 `courseFormRef`
    pub form_ref: String,
    /// 字典变量名拼接串，如 `sys_normal_disable, sys_show_hide`
    pub dict_vars: String,
    /// `useDict(...)` 的实参串，如 `'sys_normal_disable', 'sys_show_hide'`
    pub dict_literals: String,
    pub has_dict: bool,
    /// 是否存在必填校验项（没有时不能输出 `rules: {},` 里的空逗号）
    pub has_rules: bool,
}

impl GenCtx {
    /// 由表元数据 + 生成参数构造上下文。
    ///
    /// 没有主键直接报错：没有主键的表做不了按 id 的查询/更新/删除。
    pub fn build(table: &TableMeta, opt: &GenOptions) -> crate::error::Result<Self> {
        let pk_meta = table
            .pk()
            .ok_or_else(|| {
                crate::error::Error::E(format!("表 {} 没有主键，无法生成 CRUD 代码", table.table_name))
            })?;

        let class_name = to_pascal(&table.table_name);
        let business = opt
            .business_name
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| business_name(&table.table_name));
        // 表注释常以「表」结尾，去掉更顺口；实在没有就用业务名
        let function_name = opt
            .function_name
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| table.table_comment.trim().trim_end_matches('表').to_string());
        let function_name = if function_name.is_empty() {
            business.clone()
        } else {
            function_name
        };

        let mut dict_types: Vec<String> = Vec::new();
        let mut columns = Vec::with_capacity(table.columns.len());
        for c in &table.columns {
            let ov = opt.columns.get(&c.column_name);
            let html_type = ov
                .and_then(|o| o.html_type.clone())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| html_type_of(c).to_string());
            let dict_type = ov.and_then(|o| o.dict_type.clone()).unwrap_or_default();
            if !dict_type.is_empty() && !dict_types.contains(&dict_type) {
                dict_types.push(dict_type.clone());
            }
            let rust_type = rust_type_of(c).to_string();
            let ts_type = ts_type_of(&rust_type).to_string();
            let common = is_common_column(&c.column_name);

            columns.push(ColCtx {
                column_name: c.column_name.clone(),
                field: c.column_name.clone(),
                cap_field: capitalize_first(&to_camel(&c.column_name)),
                prop: to_camel(&c.column_name),
                comment: c.comment_or_default(),
                is_numeric: matches!(
                    rust_type.as_str(),
                    "i16" | "i32" | "i64" | "f32" | "f64" | "u16" | "u32" | "u64"
                ),
                is_string: rust_type == "String",
                is_datetime: rust_type.starts_with("chrono::NaiveDate"),
                is_bool: rust_type == "bool",
                ts_type,
                rust_type: rust_type.clone(),
                is_pk: c.is_pk,
                is_increment: c.is_increment,
                nullable: c.is_nullable,
                is_insert: resolve(ov.and_then(|o| o.is_insert), default_is_insert(c)),
                is_edit: resolve(ov.and_then(|o| o.is_edit), default_is_edit(c)),
                is_list: resolve(ov.and_then(|o| o.is_list), default_is_list(c)),
                is_query: resolve(ov.and_then(|o| o.is_query), default_is_query(c)),
                is_required: resolve(ov.and_then(|o| o.is_required), default_is_required(c)),
                query_type: ov
                    .and_then(|o| o.query_type.clone())
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| query_type_of(c).to_string()),
                html_type,
                dict_type,
                max_length: c.max_length.unwrap_or(0) as i64,
                is_common: common,
                now_expr: now_expr_for(&rust_type),
            });
        }

        let pk_rust_type = rust_type_of(pk_meta).to_string();
        // 主键相关的表达式先算好：pk_rust_type 稍后会 move 进 PkCtx
        let pk_new_expr_val = pk_new_expr(&pk_rust_type);
        let pk_parse_expr_val = pk_parse_expr(&pk_rust_type);
        let pk = PkCtx {
            column_name: pk_meta.column_name.clone(),
            cap_field: capitalize_first(&to_camel(&pk_meta.column_name)),
            prop: to_camel(&pk_meta.column_name),
            is_string: pk_rust_type == "String",
            rust_type: pk_rust_type,
        };
        // `pk` 稍后会 move 进结构体，先拷一份名字出来给 routes 用
        let pk_column_name = pk.column_name.clone();

        let has_create_time = table.column("create_time").is_some();
        let has_update_time = table.column("update_time").is_some();

        // 前端派生量同样提前算好：columns / dict_types 之后会被 move
        let list_var = format!("{}List", business);
        let form_ref = format!("{}FormRef", business);
        let dict_vars = dict_types.join(", ");
        let dict_literals = dict_types
            .iter()
            .map(|d| format!("'{}'", d))
            .collect::<Vec<_>>()
            .join(", ");
        let has_dict = !dict_types.is_empty();
        let has_rules = columns
            .iter()
            .any(|c| (c.is_insert || c.is_edit) && c.is_required);

        Ok(GenCtx {
            table_name: table.table_name.clone(),
            table_comment: table.table_comment.clone(),
            module_name: opt.module_name.clone(),
            business_name: business.clone(),
            function_name,
            function_author: opt.function_author.clone(),
            class_name,
            biz_file: business.clone(),
            pk,
            columns,
            api_path: format!("/{}/{}", opt.module_name, business),
            perm: format!("{}:{}", opt.module_name, business),
            rust_module: opt.module_name.clone(),
            dict_types,
            has_create_time,
            has_update_time,
            parent_menu_id: opt.parent_menu_id,
            generated_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            routes: routes_of(&business, &pk_column_name),
            pk_new_expr: pk_new_expr_val,
            pk_parse_expr: pk_parse_expr_val,
            create_by_expr: audit_value_expr(table, "create_by"),
            update_by_expr: audit_value_expr(table, "update_by"),
            list_var,
            form_ref,
            dict_vars,
            dict_literals,
            has_dict,
            has_rules,
        })
    }

    /// 用于列表查询过滤的列
    pub fn query_columns(&self) -> Vec<&ColCtx> {
        self.columns.iter().filter(|c| c.is_query).collect()
    }

    /// 列表展示列
    pub fn list_columns(&self) -> Vec<&ColCtx> {
        self.columns.iter().filter(|c| c.is_list).collect()
    }

    /// 新增表单列
    pub fn insert_columns(&self) -> Vec<&ColCtx> {
        self.columns.iter().filter(|c| c.is_insert).collect()
    }

    /// 编辑表单列（含主键）
    pub fn edit_columns(&self) -> Vec<&ColCtx> {
        self.columns.iter().filter(|c| c.is_edit || c.is_pk).collect()
    }
}

fn resolve(v: Option<bool>, default: bool) -> bool {
    v.unwrap_or(default)
}

/// 新建记录时的主键初值表达式
fn pk_new_expr(pk_rust_type: &str) -> String {
    match pk_rust_type {
        "String" => "uuid::Uuid::new_v4().to_string()".to_string(),
        "i64" => "crate::util::idseq::next_id()".to_string(),
        other => format!("crate::util::idseq::next_id() as {}", other),
    }
}

/// 把路径上的 `&str` 主键值转成实体主键类型（模板里绑到局部变量 `id` 再使用）
fn pk_parse_expr(pk_rust_type: &str) -> String {
    match pk_rust_type {
        "String" => "id.to_string()".to_string(),
        other => format!("id.parse::<{}>().unwrap_or(0)", other),
    }
}

/// 审计列的赋值表达式（以 handler 形参 `current_user: CurrentUser` 为来源，
/// 模板会按需给 add/update 挂上该形参）。表上没有这一列时返回空串。
fn audit_value_expr(table: &TableMeta, column: &str) -> String {
    match table.column(column) {
        None => String::new(),
        Some(c) => match rust_type_of(c) {
            "String" => "current_user.name.clone()".to_string(),
            "i64" => "current_user.id".to_string(),
            other => format!("current_user.id as {}", other),
        },
    }
}

/// 生成「当前时间」表达式，避免顺手写成 `.naive_local()` 让 NaiveDate 列编译不过
fn now_expr_for(rust_type: &str) -> String {
    match rust_type {
        "chrono::NaiveDateTime" => "chrono::Local::now().naive_local()".to_string(),
        "chrono::NaiveDate" => "chrono::Local::now().date_naive()".to_string(),
        "chrono::NaiveTime" => "chrono::Local::now().time()".to_string(),
        _ => String::new(),
    }
}
