//! 轻量工具函数，替代 Java 版的 `StringUtils` / `MathUtil` / `CollUtil` 中实际用到的部分。

/// 数值与比例判断
pub mod math {
    /// 比例是否为 0（或签）
    pub fn is_zero(ratio: &str) -> bool {
        to_decimal(ratio).map(|v| v == 0.0).unwrap_or(false)
    }

    /// 比例是否为 100（会签）
    pub fn is_hundred(ratio: &str) -> bool {
        to_decimal(ratio).map(|v| v == 100.0).unwrap_or(false)
    }

    /// 比例是否在 (0, 100) 之间（票签）
    pub fn is_between_zero_and_hundred(ratio: &str) -> bool {
        to_decimal(ratio).map(|v| v > 0.0 && v < 100.0).unwrap_or(false)
    }

    /// 是否数字
    pub fn is_numeric(value: &str) -> bool {
        value.trim().parse::<f64>().is_ok()
    }

    /// 比较两个数值大小：返回 -1 / 0 / 1，非数字时按字典序比较
    pub fn determine_size(left: &str, right: &str) -> i32 {
        match (to_decimal(left), to_decimal(right)) {
            (Some(l), Some(r)) => {
                if (l - r).abs() < f64::EPSILON {
                    0
                } else if l > r {
                    1
                } else {
                    -1
                }
            }
            _ => match left.cmp(right) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            },
        }
    }

    fn to_decimal(value: &str) -> Option<f64> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return None;
        }
        trimmed.parse::<f64>().ok()
    }
}

/// 按 `@@` 拆分并过滤空串，对应 Java `StringUtils.str2List(value, "@@")`
pub fn str2list(value: &str, split: &str) -> Vec<String> {
    if value.is_empty() {
        return Vec::new();
    }
    value
        .split(split)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// 取第一个非空值，对应 Java `StringUtils.emptyDefault`
pub fn empty_default(value: Option<&str>, default: &str) -> String {
    match value {
        Some(v) if !v.is_empty() => v.to_string(),
        _ => default.to_string(),
    }
}

/// 拼接字符串，对应 Java `StreamUtils.join`
pub fn join<I, S>(items: I, sep: &str) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    items.into_iter().map(|s| s.as_ref().to_string()).collect::<Vec<_>>().join(sep)
}

/// 判断切片中是否包含任意一个元素
pub fn not_contains_any(left: &[String], right: &[String]) -> bool {
    !left.iter().any(|l| right.iter().any(|r| r == l))
}
