//! 分页类型，替代 rbatis 的 `rbatis::plugin::page::{Page, PageRequest}`。
//!
//! 在 "orm 彻底改为使用 sea-orm" 迁移中，rbatis 被完全移除，这里用等价的自研结构
//! 顶替，保证 service / controller / vo 中既有的 `Page<XxxVO>` / `PageRequest` 调用
//! 与 `Page::<VO>::from(model_page)` 转换逻辑无需改动。

use serde::{Deserialize, Serialize};

/// 分页结果。字段名与 rbatis `Page` 保持一致（`records`/`total`/`size`/`current`/`pages`），
/// 以便 `vo/mod.rs` 的 `from_result` 直接读取。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub records: Vec<T>,
    pub total: u64,
    pub size: u64,
    pub current: u64,
    pub pages: u64,
}

impl<T> Page<T> {
    pub fn new(records: Vec<T>, total: u64, page_no: u64, page_size: u64) -> Self {
        let pages = if page_size == 0 {
            0
        } else {
            (total + page_size - 1) / page_size
        };
        Page {
            records,
            total,
            size: page_size,
            current: page_no,
            pages,
        }
    }
}

impl<T> Page<T> {
    /// `Page<Model>` -> `Page<VO>`，要求 `Model: Into<VO>`（各业务 VO 已实现 `From<Model>`）。
    ///
    /// 注意：不能用 blanket `impl From<Page<O>> for Page<T>`，会与标准库的
    /// 恒等 `From<T> for T` 冲突（E0119），故提供 inherent 方法。
    pub fn convert<O>(self) -> Page<O>
    where
        T: Into<O>,
    {
        Page {
            records: self.records.into_iter().map(Into::into).collect(),
            total: self.total,
            size: self.size,
            current: self.current,
            pages: self.pages,
        }
    }
}

/// 分页请求。对应 rbatis `PageRequest`，字段含 `page_no`/`page_size`/`pageoffset`。
#[derive(Clone, Debug, Default)]
pub struct PageRequest {
    pub page_no: u64,
    pub page_size: u64,
    pub pageoffset: u64,
}

impl PageRequest {
    pub fn new(page_no: u64, page_size: u64) -> Self {
        PageRequest {
            page_no,
            page_size,
            pageoffset: page_no.saturating_sub(1) * page_size,
        }
    }

    /// 覆盖 offset（部分自定义查询需手动计算 postgresql 的 offset）。
    pub fn set_pageoffset(mut self, offset: u64) -> Self {
        self.pageoffset = offset;
        self
    }
}
