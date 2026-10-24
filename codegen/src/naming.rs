//! 命名工具：snake_case / PascalCase / camelCase / kebab-case 互转。
//!
//! 旧版本依赖 `Inflector`，但它解决的是通用英语词形转换，对本工具只需要几个固定规则，
//! 且 Inflector 已多年未更新。这里自己实现，少一个依赖、行为可控（尤其是数字下标的 table_name 拆分）。

/// `snake_case` -> `PascalCase`（表名 -> 结构体/类名）
///
/// `oms_order_item` -> `OmsOrderItem`
pub fn to_pascal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = true;
    for ch in s.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
            upper = true;
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

/// `snake_case` -> `camelCase`（列名 -> 前端属性名）
pub fn to_camel(s: &str) -> String {
    let pascal = to_pascal(s);
    let mut it = pascal.chars();
    match it.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + it.as_str(),
    }
}

/// `PascalCase` 首字母大写版本 -> 方法/模块名用的 snake
pub fn to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// 去掉表名的业务前缀，得到业务名：`oms_order_item` -> `orderItem`
///
/// 与旧版 `get_business_name` 的差别：旧版遇到无下划线的表名会返回 Err，
/// 导致 `demo` 这类单段表名直接生成失败；这里退化为整表名，更健壮。
pub fn business_name(table_name: &str) -> String {
    match table_name.find('_') {
        Some(idx) if idx + 1 < table_name.len() => to_camel(&table_name[idx + 1..]),
        _ => to_camel(table_name),
    }
}

/// 大写首字母（组 Fields 常量名时用）
pub fn capitalize_first(s: &str) -> String {
    let mut it = s.chars();
    match it.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + it.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_conversions() {
        assert_eq!(to_pascal("oms_order_item"), "OmsOrderItem");
        assert_eq!(to_camel("course_name"), "courseName");
        assert_eq!(to_snake("OmsOrderItem"), "oms_order_item");
        assert_eq!(business_name("oms_order_item"), "orderItem");
        assert_eq!(business_name("demo"), "demo");
        assert_eq!(capitalize_first("courseName"), "CourseName");
    }
}
