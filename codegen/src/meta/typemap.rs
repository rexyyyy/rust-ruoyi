//! 数据库类型 -> Rust 类型 / 前端控件类型（PostgreSQL + MySQL）。
//!
//! 旧版 `get_type_from_db_type` 用 `find("int8")` 这种子串匹配，顺序写死且脆弱：
//! `timestamp` 会被前面的 `date` 分支抢走、`numeric` 的判断依赖一个没用上的 `dig` 参数。
//! 这里改成按 `information_schema.columns` 里的精确类型名匹配，行为可预期。
//!
//! 类型名来源：PG 取 `udt_name`（int8/varchar/timestamp ...），
//! MySQL 取 `data_type`（bigint/varchar/datetime ...），两套名字在同一张表里并列匹配。

use super::column::ColumnMeta;

/// 数据库类型名（PG `udt_name` / MySQL `data_type`）-> Rust 类型字符串。
///
/// 约定与 backend 的实体写法保持一致：
/// - 非主键列在 sea-orm 实体里统一为 `Option<T>`，`T` 即本函数返回值；
/// - 主键列不加 `Option`，直接使用本函数返回值。
pub fn rust_type_of(col: &ColumnMeta) -> &'static str {
    match col.column_type.as_str() {
        // PG
        "int2" | "smallint" => "i16",
        "int4" | "integer" | "serial" => "i32",
        "int8" | "bigint" | "bigserial" => "i64",
        "float4" | "real" => "f32",
        "float8" | "double precision" => "f64",
        // MySQL（smallint/bigint/decimal/date/time/json 与 PG 同名，已在上/下方命中）
        "tinyint" => "i8",
        "mediumint" | "int" | "integer unsigned" => "i32",
        "double" => "f64",
        "datetime" => "chrono::NaiveDateTime",
        "year" => "i16",
        // numeric 的小数位为 0 时按整型处理（金额表常见 numeric(18,0)）；
        // 有小数位时映射成 f64 —— 精确金额应改用 rust_decimal，
        // 但那需要在 backend 打开 sea-orm 的 with-rust_decimal feature，
        // 生成器默认不给宿主项目加额外开关，工程上按需手工替换即可。
        "numeric" | "decimal" => {
            if col.numeric_scale.unwrap_or(0) > 0 {
                "f64"
            } else {
                "i64"
            }
        }
        "bool" | "boolean" => "bool",
        "date" => "chrono::NaiveDate",
        "timestamp" | "timestamptz" => "chrono::NaiveDateTime",
        "time" => "chrono::NaiveTime",
        "json" | "jsonb" => "serde_json::Value",
        "bytea" => "Vec<u8>",
        // MySQL 二进制族（bit 也按字节处理，tinyint(1) 已由 tinyint 分支覆盖）
        "bit" | "binary" | "varbinary" | "tinyblob" | "blob" | "mediumblob" | "longblob" => {
            "Vec<u8>"
        }
        _ => "String",
    }
}

/// 是否需要在 entity 文件顶部额外 `use`（chrono / serde_json 两类不是 Rust 原生类型）
pub fn needs_chrono_use(rust_type: &str) -> bool {
    rust_type.starts_with("chrono::")
}

/// 前端表单控件类型
///
/// 只做保守猜测，用户在「编辑」页仍可逐列调整后保存（保存值会写回 gen_table_column）。
pub fn html_type_of(col: &ColumnMeta) -> &'static str {
    let name = col.column_name.to_lowercase();
    let t = rust_type_of(col);
    if name.contains("image") || name.contains("img") || name.contains("pic") {
        return "imageUpload";
    }
    if name.contains("file") || name.contains("attach") {
        return "fileUpload";
    }
    if t.starts_with("chrono::NaiveDate") {
        return "datetime";
    }
    if name.ends_with("status") || name.ends_with("type") || name.ends_with("state")
        || name.ends_with("flag") || name.ends_with("scope")
    {
        return "radio";
    }
    if t == "bool" {
        return "radio";
    }
    if t == "String" && col.max_length.unwrap_or(0) >= 500 {
        return "textarea";
    }
    "input"
}

/// 查询方式（SQL 比较符）
pub fn query_type_of(col: &ColumnMeta) -> &'static str {
    if rust_type_of(col) == "String" {
        "LIKE"
    } else if html_type_of(col) == "datetime" {
        "BETWEEN"
    } else {
        "EQ"
    }
}

/// 是否作为列表/表单字段参与生成
pub fn is_common_column(name: &str) -> bool {
    matches!(name, "create_by" | "create_time" | "update_by" | "update_time")
}

/// 默认是否出现在查询区
///
/// 时间列默认不进查询区：前端日期区间会传 `begin*`/`end*` 两个参数，
/// 与本工具生成的单一字段查询不匹配，宁可不生成也不要产出跑不通的查询条件。
/// 需要日期区间过滤时，在生成后手工扩展 PageDTO 与 service。
pub fn default_is_query(col: &ColumnMeta) -> bool {
    if col.is_pk || is_common_column(&col.column_name) {
        return false;
    }
    if rust_type_of(col).starts_with("chrono::") {
        return false;
    }
    let name = col.column_name.to_lowercase();
    name.ends_with("name") || name.ends_with("code") || name.ends_with("type") || name == "status"
}

/// 默认是否出现在新增表单
///
/// 主键一律排除：本工具生成的新增接口自己用雪花 ID / uuid 造主键，
/// 客户端传什么都以服务端为准（避免前端伪造主键）。若主键是业务键
/// （如用 varchar 编码当主键），在生成配置里把该列的 `is_insert` 打开即可。
pub fn default_is_insert(col: &ColumnMeta) -> bool {
    !col.is_pk && !is_common_column(&col.column_name)
}

/// 默认是否出现在编辑表单（主键作为隐藏项需要带上）
pub fn default_is_edit(col: &ColumnMeta) -> bool {
    !(col.is_pk && col.is_increment) && !is_common_column(&col.column_name)
}

/// 默认是否出现在列表
pub fn default_is_list(col: &ColumnMeta) -> bool {
    !(col.is_pk && col.is_increment)
}

/// 默认是否必填（非空且非主键自增、非审计列）
pub fn default_is_required(col: &ColumnMeta) -> bool {
    !col.is_nullable && !col.is_pk && !is_common_column(&col.column_name)
}
