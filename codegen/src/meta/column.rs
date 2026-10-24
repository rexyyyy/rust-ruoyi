//! 表 / 列元数据模型。
//!
//! 这层是「从数据库读出来的原始事实」，不含任何业务生成策略；
//! 生成策略在 [`crate::meta::typemap`] 与 [`crate::gen`] 里，
//! 这样替换模板或调整策略都不影响元数据读取。

use serde::{Deserialize, Serialize};

/// 一张表的原始元数据
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TableMeta {
    pub table_name: String,
    pub table_comment: String,
    pub columns: Vec<ColumnMeta>,
}

impl TableMeta {
    /// 主键列。没有主键的表无法生成 CRUD 代码。
    pub fn pk(&self) -> Option<&ColumnMeta> {
        self.columns.iter().find(|c| c.is_pk)
    }

    pub fn column(&self, name: &str) -> Option<&ColumnMeta> {
        self.columns.iter().find(|c| c.column_name == name)
    }
}

/// 一列的元数据
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ColumnMeta {
    pub column_name: String,
    /// PostgreSQL `udt_name`（int8 / varchar / timestamp ...）
    pub column_type: String,
    pub column_comment: String,
    pub is_pk: bool,
    pub is_nullable: bool,
    /// 是否自增（serial / identity），自增主键在新增表单里不出现
    pub is_increment: bool,
    pub ordinal: i32,
    pub max_length: Option<i32>,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
}

impl ColumnMeta {
    /// 没写列注释时给一个可读兜底，避免前端表单 label 全空
    pub fn comment_or_default(&self) -> String {
        if self.column_comment.trim().is_empty() {
            format!("列{}", self.ordinal)
        } else {
            self.column_comment.clone()
        }
    }
}

/// 「从数据库里挑一张表」时的候选项（对应前端「导入表」列表）
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbTableRow {
    pub table_name: String,
    pub table_comment: String,
    /// PostgreSQL 不提供每表创建/更新时间，占位为 null（对齐前端列定义）
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
