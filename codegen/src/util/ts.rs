//! Rust 类型 -> TypeScript 类型。用在前端 `types.ts` 与表单控件。
pub fn ts_type_of(rust_type: &str) -> &'static str {
    match rust_type {
        "bool" => "boolean",
        "i16" | "i32" | "i64" | "u16" | "u32" | "u64" | "f32" | "f64" => "number",
        "String" => "string",
        // chrono 在 JSON 里是字符串；NaiveTime 同理
        "chrono::NaiveDateTime" | "chrono::NaiveDate" | "chrono::NaiveTime" => "string",
        _ => "any",
    }
}
