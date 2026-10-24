//! gen_table / gen_table_column 实体与 DTO。
//!
//! 字段命名保持与官方 RuoYi-Vue-Plus 一致（java_field 列承载 rust 字段名，
//! 前端编辑页字段名不动，降低联调成本）。

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use sea_orm::entity::prelude::*;

/// 代码生成业务表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gen_table")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub table_id: i64,
    pub data_name: Option<String>,
    pub table_name: Option<String>,
    pub table_comment: Option<String>,
    pub class_name: Option<String>,
    pub tpl_category: Option<String>,
    pub package_name: Option<String>,
    pub module_name: Option<String>,
    pub business_name: Option<String>,
    pub function_name: Option<String>,
    pub function_author: Option<String>,
    pub gen_type: Option<String>,
    pub gen_path: Option<String>,
    pub options: Option<String>,
    pub sub_table_name: Option<String>,
    pub sub_table_fk_name: Option<String>,
    pub create_dept: Option<i64>,
    pub remark: Option<String>,
    pub create_by: Option<i64>,
    pub create_time: Option<NaiveDateTime>,
    pub update_by: Option<i64>,
    pub update_time: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub use Model as GenTable;
pub use Entity as GenTableEntity;
pub use Column as GenTableColumnEnum;
pub use ActiveModel as GenTableActiveModel;

/// 代码生成业务表字段（独立模块避免与 gen_table 的 derive 类型冲突）
pub mod gen_table_column {
    use chrono::NaiveDateTime;
    use sea_orm::entity::prelude::*;

    /// 代码生成业务表字段
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "gen_table_column")]
    #[serde(rename_all = "camelCase")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub column_id: i64,
        pub table_id: i64,
        pub column_name: Option<String>,
        pub column_comment: Option<String>,
        pub column_type: Option<String>,
        pub java_type: Option<String>,
        pub java_field: Option<String>,
        pub is_pk: Option<String>,
        pub is_increment: Option<String>,
        pub is_required: Option<String>,
        pub is_insert: Option<String>,
        pub is_edit: Option<String>,
        pub is_list: Option<String>,
        pub is_query: Option<String>,
        pub query_type: Option<String>,
        pub html_type: Option<String>,
        pub dict_type: Option<String>,
        pub sort: Option<i32>,
        pub create_dept: Option<i64>,
        pub create_by: Option<i64>,
        pub create_time: Option<NaiveDateTime>,
        pub update_by: Option<i64>,
        pub update_time: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}

    pub use Model as GenTableColumn;
    pub use Entity as GenTableColumnEntity;
    pub use Column as GenTableColumnEnum;
    pub use ActiveModel as GenTableColumnActiveModel;
}

/// 详情返回结构（含 transient 字段），与前端 GenTableVO { info, rows, tables } 对齐。
///
/// `parent_menu_id` 在官方 RuoYi-Vue-Plus 中为 `@TableField(exist = false)` 的
/// 瞬态字段，源自 `options` JSON，并非数据库列，故单独定义 DTO。
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenTableInfo {
    pub table_id: i64,
    pub data_name: Option<String>,
    pub table_name: Option<String>,
    pub table_comment: Option<String>,
    pub sub_table_name: Option<String>,
    pub sub_table_fk_name: Option<String>,
    pub class_name: Option<String>,
    pub tpl_category: Option<String>,
    pub package_name: Option<String>,
    pub module_name: Option<String>,
    pub business_name: Option<String>,
    pub function_name: Option<String>,
    pub function_author: Option<String>,
    pub gen_type: Option<String>,
    pub gen_path: Option<String>,
    pub options: Option<String>,
    pub create_dept: Option<i64>,
    pub create_by: Option<i64>,
    pub create_time: Option<NaiveDateTime>,
    pub update_by: Option<i64>,
    pub update_time: Option<NaiveDateTime>,
    pub remark: Option<String>,
    /// 瞬态：来自 options JSON 的 parentMenuId（官方为 exist=false 字段）
    pub parent_menu_id: Option<i64>,
}

impl From<GenTable> for GenTableInfo {
    fn from(m: GenTable) -> Self {
        let pid = m
            .options
            .as_deref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
            .and_then(|v| v.get("parentMenuId").and_then(|x| x.as_i64()))
            .or(Some(0));
        GenTableInfo {
            table_id: m.table_id,
            data_name: m.data_name,
            table_name: m.table_name,
            table_comment: m.table_comment,
            sub_table_name: m.sub_table_name,
            sub_table_fk_name: m.sub_table_fk_name,
            class_name: m.class_name,
            tpl_category: m.tpl_category,
            package_name: m.package_name,
            module_name: m.module_name,
            business_name: m.business_name,
            function_name: m.function_name,
            function_author: m.function_author,
            gen_type: m.gen_type,
            gen_path: m.gen_path,
            options: m.options,
            create_dept: m.create_dept,
            create_by: m.create_by,
            create_time: m.create_time,
            update_by: m.update_by,
            update_time: m.update_time,
            remark: m.remark,
            parent_menu_id: pid,
        }
    }
}

/// 详情 VO：与前端 GenTableVO { info, rows, tables } 对齐
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenTableVO {
    pub info: GenTableInfo,
    pub rows: Vec<gen_table_column::Model>,
    pub tables: Vec<GenTable>,
}

/// 导入参数（POST /tool/gen/importTable 的 query 参数）
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImportTableQuery {
    pub tables: String,
    #[serde(default)]
    pub data_name: Option<String>,
}

/// 列表查询参数
#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenQuery {
    #[serde(rename(deserialize = "pageNum"), default)]
    pub page_no: Option<u64>,
    #[serde(rename(deserialize = "pageSize"), default)]
    pub page_size: Option<u64>,
    #[serde(default)]
    pub table_name: Option<String>,
    #[serde(default)]
    pub table_comment: Option<String>,
    #[serde(default)]
    pub data_name: Option<String>,
}

/// 修改配置的列（与前端 DbColumnForm 对齐，未列出的字段不参与更新）
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenColumnForm {
    pub column_id: Option<i64>,
    pub table_id: Option<i64>,
    pub column_name: Option<String>,
    pub column_comment: Option<String>,
    #[serde(default)]
    pub java_field: Option<String>,
    #[serde(default)]
    pub is_pk: Option<String>,
    #[serde(default)]
    pub is_increment: Option<String>,
    #[serde(default)]
    pub is_required: Option<String>,
    #[serde(default)]
    pub is_insert: Option<String>,
    #[serde(default)]
    pub is_edit: Option<String>,
    #[serde(default)]
    pub is_list: Option<String>,
    #[serde(default)]
    pub is_query: Option<String>,
    #[serde(default)]
    pub query_type: Option<String>,
    #[serde(default)]
    pub html_type: Option<String>,
    #[serde(default)]
    pub dict_type: Option<String>,
    #[serde(default)]
    pub sort: Option<i32>,
}

/// 修改配置请求（与前端 DbTableForm 对齐）
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenTableForm {
    pub table_id: i64,
    #[serde(default)]
    pub table_name: Option<String>,
    #[serde(default)]
    pub table_comment: Option<String>,
    #[serde(default)]
    pub class_name: Option<String>,
    #[serde(default)]
    pub module_name: Option<String>,
    #[serde(default)]
    pub business_name: Option<String>,
    #[serde(default)]
    pub function_name: Option<String>,
    #[serde(default)]
    pub function_author: Option<String>,
    #[serde(default)]
    pub parent_menu_id: Option<i64>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default)]
    pub options: Option<String>,
    #[serde(default)]
    pub columns: Vec<GenColumnForm>,
    /// 前端 DbTableForm.params.parentMenuId 也会带上，兜底取一次
    #[serde(default)]
    pub params: Option<GenTableParams>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenTableParams {
    #[serde(default, deserialize_with = "de_opt_string_flexible")]
    pub parent_menu_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string_flexible")]
    pub tree_code: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string_flexible")]
    pub tree_name: Option<String>,
    #[serde(default, deserialize_with = "de_opt_string_flexible")]
    pub tree_parent_code: Option<String>,
}

/// 前端回传的瞬态参数值类型不定：parentMenuId 在 options JSON 里是数字，
/// tree* 可能为 null；宽容接受 string / number / bool / null。
fn de_opt_string_flexible<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    match Option::<serde_json::Value>::deserialize(deserializer)? {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::String(s)) => Ok(Some(s)),
        Some(serde_json::Value::Number(n)) => Ok(Some(n.to_string())),
        Some(serde_json::Value::Bool(b)) => Ok(Some(b.to_string())),
        Some(other) => Err(D::Error::custom(format!("unexpected value: {other}"))),
    }
}
