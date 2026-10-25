//! 统一的时间序列化出口处理
//!
//! chrono 的 `NaiveDateTime` 默认被 serde 写成 ISO 形态（`2026-11-02T09:00:00`，
//! 来自数据库时还会带小数秒 `2026-10-25T19:36:35.700889`），而若依前端所有表格、
//! 表单回填都约定 `yyyy-MM-dd HH:mm:ss`。逐个 VO 字段挂 `#[serde(with)]` 要改
//! 几十处且新增字段极易漏，所以统一在响应出口处做一次字符串级改写：
//! 只处理「双引号包裹且严格符合 ISO 日期时间形态」的整体值，普通文本不会命中。

use serde::Serialize;
use std::borrow::Cow;

/// 与前端约定一致的时间格式
pub const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";

/// `NaiveDateTime` → `yyyy-MM-dd HH:mm:ss`（非响应出口、需要手工格式化时用）
pub fn format_datetime(dt: &chrono::NaiveDateTime) -> String {
    dt.format(DATETIME_FMT).to_string()
}

/// 序列化 + 时间格式归一，所有 JSON 响应的唯一出口
pub fn to_json_string<T: Serialize + ?Sized>(value: &T) -> String {
    let raw = serde_json::to_string(value).unwrap_or_else(|e| {
        log::error!("[ruoyi_rust] 响应序列化失败: {e}");
        r#"{"code":500,"msg":"响应序列化失败"}"#.to_string()
    });
    normalize_json(&raw).into_owned()
}

/// 把 JSON 文本里的 ISO 时间改成空格分隔格式，并去掉小数秒
///
/// 没有命中时返回 `Cow::Borrowed`，不产生额外分配。
pub fn normalize_json(src: &str) -> Cow<'_, str> {
    // ISO 时间必含 'T'，先做一次廉价短路
    if src.find('T').is_none() {
        return Cow::Borrowed(src);
    }
    let b = src.as_bytes();
    let mut out: Option<String> = None;
    // 已复制到 out 的字节位置
    let mut copied = 0usize;
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'"' {
            if let Some(close) = match_datetime(b, i + 1) {
                let buf = out.get_or_insert_with(|| String::with_capacity(src.len()));
                buf.push_str(&src[copied..i]);
                // ISO 串全为 ASCII，按字节切片再拼接不会破坏字符边界
                let seg = &src[i + 1..i + 20];
                buf.push('"');
                buf.push_str(&seg[..10]);
                buf.push(' ');
                buf.push_str(&seg[11..19]);
                buf.push('"');
                i = close + 1;
                copied = i;
                continue;
            }
        }
        i += 1;
    }
    match out {
        None => Cow::Borrowed(src),
        Some(mut s) => {
            s.push_str(&src[copied..]);
            Cow::Owned(s)
        }
    }
}

/// 判断 `s` 起是否为 `YYYY-MM-DDTHH:MM:SS[.fff]` 且紧跟闭引号，返回闭引号下标
fn match_datetime(b: &[u8], s: usize) -> Option<usize> {
    if s + 19 > b.len() {
        return None;
    }
    let digit = |i: usize| b[i].is_ascii_digit();
    let shaped = digit(s)
        && digit(s + 1)
        && digit(s + 2)
        && digit(s + 3)
        && b[s + 4] == b'-'
        && digit(s + 5)
        && digit(s + 6)
        && b[s + 7] == b'-'
        && digit(s + 8)
        && digit(s + 9)
        && b[s + 10] == b'T'
        && digit(s + 11)
        && digit(s + 12)
        && b[s + 13] == b':'
        && digit(s + 14)
        && digit(s + 15)
        && b[s + 16] == b':'
        && digit(s + 17)
        && digit(s + 18);
    if !shaped {
        return None;
    }
    let mut j = s + 19;
    // 小数秒可有可无，出现了就必须是至少一位数字
    if j < b.len() && b[j] == b'.' {
        j += 1;
        let mut n = 0;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
            n += 1;
        }
        if n == 0 {
            return None;
        }
    }
    if j < b.len() && b[j] == b'"' {
        Some(j)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrite_iso_with_fractional_seconds() {
        let src = r#"{"createTime":"2026-10-25T19:36:35.700889"}"#;
        assert_eq!(normalize_json(src), r#"{"createTime":"2026-10-25 19:36:35"}"#);
    }

    #[test]
    fn rewrite_iso_without_fraction() {
        let src = r#"{"rows":[{"start_date":"2026-12-01T09:00:00"}],"total":1}"#;
        assert_eq!(normalize_json(src), r#"{"rows":[{"start_date":"2026-12-01 09:00:00"}],"total":1}"#);
    }

    #[test]
    fn plain_text_untouched() {
        // 含 T 但不是完整日期时间形态；以及嵌在句子中间的 ISO 片段
        let src = r#"{"remark":"TIME 2026-11-02T09:00:00 之前","name":"Tom","ts":"2026-11-02T09:00:00+08:00"}"#;
        assert_eq!(normalize_json(src), src);
    }

    #[test]
    fn no_datetime_no_copy() {
        let src = r#"{"code":200,"msg":"操作成功"}"#;
        assert!(matches!(normalize_json(src), Cow::Borrowed(_)));
    }

    #[test]
    fn date_only_is_not_rewritten() {
        let src = r#"{"day":"2026-11-02","ts":"2026-11-02T09:00:00"}"#;
        assert_eq!(normalize_json(src), r#"{"day":"2026-11-02","ts":"2026-11-02 09:00:00"}"#);
    }
}
