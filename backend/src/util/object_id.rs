//! 替代 rbatis 的 `ObjectId`：生成 24 位 hex 唯一 id（varchar 主键用）。
//! 使用 uuid 实现，等价于原 `crate::util::object_id::new_object_id()` 的语义。

use uuid::Uuid;

/// 生成唯一 id 字符串（形如 `9f8b7c6d5e4f3a2b1c0d2e3f`）。
pub fn new_object_id() -> String {
    Uuid::new_v4().to_string().replace('-', "")
}
