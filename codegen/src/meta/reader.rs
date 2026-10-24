//! 从数据库读取表 / 列元数据（PostgreSQL / MySQL 双方言）。
//!
//! 旧版本用一条横跨 `pg_class / pg_attribute / information_schema.columns / pg_description`
//! 的手写 SQL，再用 `rbs::Value` 的 `as_string()/as_i64()` 逐个取值，类型全靠猜。
//! 这里用 sea-orm 的原生查询 + `try_get::<T>` 精确取类型，失败即报错而不是 panic。
//!
//! 方言差异集中在两个地方：
//! 1. **系统表**：PG 用 `pg_namespace/pg_class/pg_description/pg_constraint` 拿注释与主键，
//!    MySQL 的 `information_schema` 自带 `table_comment/column_comment/column_key/extra`，
//!    一条 SQL 即可，不需要关联系统表；
//! 2. **数字列类型**：MySQL 的 `information_schema.columns` 里 `ordinal_position` /
//!    `character_maximum_length` / `numeric_precision` 都是 `bigint unsigned`，
//!    按 `i32` 解码会类型不匹配，故 MySQL 分支先取 `i64` 再收窄。

use sea_orm::{ConnectionTrait, DbBackend, Statement};

use super::column::{ColumnMeta, DbTableRow, TableMeta};
use crate::error::Result;
use crate::pool;

/// PostgreSQL 默认 schema；MySQL 下该常量会被忽略（改为当前库 `DATABASE()`）
pub const DEFAULT_SCHEMA: &str = "public";

/// 当前连接的真实后端，决定走 PG 系统表还是 MySQL information_schema 分支
fn backend() -> DbBackend {
    pool!().get_database_backend()
}

/// MySQL 分支的 schema 入参：调用方传的是 PG 风格 `public` 时折算为 NULL，
/// 交给 SQL 里的 `COALESCE(?, DATABASE())` 兜底成当前库。
fn mysql_schema_arg(schema: &str) -> Option<String> {
    if schema.is_empty() || schema.eq_ignore_ascii_case("public") {
        None
    } else {
        Some(schema.to_string())
    }
}

/// 列出可导入的物理表（已纳入 gen_table 的表由上层过滤）
pub async fn list_tables(schema: &str) -> Result<Vec<DbTableRow>> {
    let backend = backend();
    let rows = if backend == DbBackend::MySql {
        let sql = r#"
            SELECT t.table_name                       AS table_name,
                   COALESCE(t.table_comment, '')       AS table_comment
            FROM information_schema.tables t
            WHERE t.table_schema = COALESCE(?, DATABASE())
              AND t.table_type = 'BASE TABLE'
            ORDER BY t.table_name
        "#;
        pool!()
            .query_all(Statement::from_sql_and_values(
                backend,
                sql,
                vec![mysql_schema_arg(schema).into()],
            ))
            .await?
    } else {
        let sql = r#"
            SELECT t.table_name AS table_name,
                   COALESCE(d.description, '') AS table_comment
            FROM information_schema.tables t
            LEFT JOIN pg_namespace n ON n.nspname = t.table_schema
            LEFT JOIN pg_class c ON c.relname = t.table_name AND c.relnamespace = n.oid
            LEFT JOIN pg_description d ON d.objoid = c.oid AND d.objsubid = 0
            WHERE t.table_schema = $1 AND t.table_type = 'BASE TABLE'
            ORDER BY t.table_name
        "#;
        pool!()
            .query_all(Statement::from_sql_and_values(
                DbBackend::Postgres,
                sql,
                vec![schema.into()],
            ))
            .await?
    };

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(DbTableRow {
            table_name: r.try_get("", "table_name")?,
            table_comment: r.try_get::<String>("", "table_comment")?.trim().to_string(),
            create_time: None,
            update_time: None,
        });
    }
    Ok(out)
}

/// 读取单张表的完整列定义
pub async fn load_table(schema: &str, table_name: &str) -> Result<TableMeta> {
    let columns = load_columns(schema, table_name).await?;
    Ok(TableMeta {
        table_name: table_name.to_string(),
        table_comment: load_table_comment(schema, table_name).await?,
        columns,
    })
}

/// 只取列定义（同步数据库结构时会用到）
pub async fn load_columns(schema: &str, table_name: &str) -> Result<Vec<ColumnMeta>> {
    let backend = backend();
    if backend == DbBackend::MySql {
        return load_columns_mysql(schema, table_name).await;
    }
    let sql = r#"
        SELECT c.column_name                     AS column_name,
               c.udt_name                        AS column_type,
               c.is_nullable                     AS is_nullable,
               c.ordinal_position                AS ordinal,
               c.character_maximum_length        AS max_length,
               c.numeric_precision               AS numeric_precision,
               c.numeric_scale                   AS numeric_scale,
               COALESCE(pd.description, '')      AS column_comment,
               COALESCE((SELECT count(*) > 0
                         FROM pg_constraint pc
                         WHERE pc.conrelid = cl.oid
                           AND pc.contype = 'p'
                           AND array_position(pc.conkey::int2[], attr.attnum) IS NOT NULL),
                        false)                   AS is_pk,
               COALESCE(c.is_identity, 'NO') = 'YES'
                 OR COALESCE(c.column_default, '') LIKE 'nextval(%' AS is_increment
        FROM information_schema.columns c
        LEFT JOIN pg_namespace ns ON ns.nspname = c.table_schema
        LEFT JOIN pg_class cl ON cl.relname = c.table_name AND cl.relnamespace = ns.oid
        LEFT JOIN pg_attribute attr ON attr.attrelid = cl.oid AND attr.attname = c.column_name
        LEFT JOIN pg_description pd ON pd.objoid = cl.oid AND pd.objsubid = attr.attnum
        WHERE c.table_schema = $1 AND c.table_name = $2
        ORDER BY c.ordinal_position
    "#;
    let rows = pool!()
        .query_all(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            vec![schema.into(), table_name.into()],
        ))
        .await?;

    if rows.is_empty() {
        return Err(format!("表 {}.{} 不存在或无列定义", schema, table_name).into());
    }

    let mut cols = Vec::with_capacity(rows.len());
    for r in rows {
        cols.push(ColumnMeta {
            column_name: r.try_get("", "column_name")?,
            column_type: r.try_get("", "column_type")?,
            column_comment: r
                .try_get::<String>("", "column_comment")?
                .trim()
                .to_string(),
            is_pk: r.try_get("", "is_pk")?,
            is_nullable: r.try_get::<String>("", "is_nullable")? == "YES",
            is_increment: r.try_get("", "is_increment")?,
            ordinal: r.try_get("", "ordinal")?,
            max_length: r.try_get::<Option<i32>>("", "max_length")?,
            numeric_precision: r.try_get::<Option<i32>>("", "numeric_precision")?,
            numeric_scale: r.try_get::<Option<i32>>("", "numeric_scale")?,
        });
    }
    Ok(cols)
}

/// MySQL 分支：注释/主键/自增都在 `information_schema.columns` 里，无需关联系统表。
///
/// 两个解码注意点：
/// - 主键与自增用 `CASE WHEN ... THEN '1' ELSE '0'` 输出字符串再比较，
///   避免 `IF()` 产生的整数与 Rust `bool` 解码类型不匹配；
/// - `ORDINAL_POSITION` 是 `int unsigned`、`NUMERIC_PRECISION/SCALE` 是 `bigint unsigned`，
///   sqlx 会按“列申明类型”严格校验，统一 `CAST(... AS SIGNED)` 成 signed bigint 再取 `i64`。
async fn load_columns_mysql(schema: &str, table_name: &str) -> Result<Vec<ColumnMeta>> {
    let sql = r#"
        SELECT c.column_name                                  AS column_name,
               c.data_type                                    AS column_type,
               c.is_nullable                                  AS is_nullable,
               CAST(c.ordinal_position AS SIGNED)             AS ordinal,
               CAST(c.character_maximum_length AS SIGNED)     AS max_length,
               CAST(c.numeric_precision AS SIGNED)            AS numeric_precision,
               CAST(c.numeric_scale AS SIGNED)                AS numeric_scale,
               COALESCE(c.column_comment, '')                 AS column_comment,
               CASE WHEN c.column_key = 'PRI' THEN '1' ELSE '0' END AS is_pk,
               CASE WHEN c.extra LIKE '%auto_increment%' THEN '1' ELSE '0' END AS is_increment
        FROM information_schema.columns c
        WHERE c.table_schema = COALESCE(?, DATABASE()) AND c.table_name = ?
        ORDER BY c.ordinal_position
    "#;
    let rows = pool!()
        .query_all(Statement::from_sql_and_values(
            DbBackend::MySql,
            sql,
            vec![mysql_schema_arg(schema).into(), table_name.into()],
        ))
        .await?;

    if rows.is_empty() {
        return Err(format!("表 {} 不存在或无列定义", table_name).into());
    }

    let mut cols = Vec::with_capacity(rows.len());
    for r in rows {
        // information_schema 的数字列是 bigint unsigned，先按 i64 取再收窄
        let narrow = |v: Option<i64>| v.map(|n| n as i32);
        cols.push(ColumnMeta {
            column_name: r.try_get("", "column_name")?,
            column_type: r.try_get("", "column_type")?,
            column_comment: r
                .try_get::<String>("", "column_comment")?
                .trim()
                .to_string(),
            is_pk: r.try_get::<String>("", "is_pk")? == "1",
            is_nullable: r.try_get::<String>("", "is_nullable")? == "YES",
            is_increment: r.try_get::<String>("", "is_increment")? == "1",
            ordinal: r.try_get::<i64>("", "ordinal")? as i32,
            max_length: narrow(r.try_get::<Option<i64>>("", "max_length")?),
            numeric_precision: narrow(r.try_get::<Option<i64>>("", "numeric_precision")?),
            numeric_scale: narrow(r.try_get::<Option<i64>>("", "numeric_scale")?),
        });
    }
    Ok(cols)
}

async fn load_table_comment(schema: &str, table_name: &str) -> Result<String> {
    let backend = backend();
    let rows = if backend == DbBackend::MySql {
        let sql = r#"
            SELECT COALESCE(t.table_comment, '') AS table_comment
            FROM information_schema.tables t
            WHERE t.table_schema = COALESCE(?, DATABASE()) AND t.table_name = ?
            LIMIT 1
        "#;
        pool!()
            .query_all(Statement::from_sql_and_values(
                backend,
                sql,
                vec![mysql_schema_arg(schema).into(), table_name.into()],
            ))
            .await?
    } else {
        let sql = r#"
            SELECT COALESCE(d.description, '') AS table_comment
            FROM information_schema.tables t
            LEFT JOIN pg_namespace n ON n.nspname = t.table_schema
            LEFT JOIN pg_class c ON c.relname = t.table_name AND c.relnamespace = n.oid
            LEFT JOIN pg_description d ON d.objoid = c.oid AND d.objsubid = 0
            WHERE t.table_schema = $1 AND t.table_name = $2
            LIMIT 1
        "#;
        pool!()
            .query_all(Statement::from_sql_and_values(
                DbBackend::Postgres,
                sql,
                vec![schema.into(), table_name.into()],
            ))
            .await?
    };
    Ok(match rows.into_iter().next() {
        Some(r) => r.try_get::<String>("", "table_comment")?.trim().to_string(),
        None => String::new(),
    })
}
