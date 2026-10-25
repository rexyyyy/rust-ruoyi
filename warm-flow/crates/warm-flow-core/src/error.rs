//! 错误类型。对应 Java 版 `FlowException` 与 `AssertUtil` 抛出的校验失败。

use serde::Serialize;

pub type Result<T> = std::result::Result<T, FlowError>;

/// 流程引擎错误。
///
/// 业务校验失败统一用 [`FlowError::Biz`]，HTTP 层映射为 `code = 500` + 原始中文文案，
/// 与 Java 版前端提示保持一致；持久层错误保留底层信息，便于排查。
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    /// 业务校验失败（Java 的 `AssertUtil`）
    #[error("{0}")]
    Biz(String),
    /// 数据不存在
    #[error("{0}")]
    NotFound(String),
    /// 数据库错误
    #[error("数据库操作失败: {0}")]
    Db(#[from] sea_orm_like::DbError),
}

/// 为避免 core 依赖 SeaORM，用一个极薄的错误包装层承接 `warm-flow-orm` 的转换。
pub mod sea_orm_like {
    /// 持久层错误载体，`warm-flow-orm` 会把 `sea_orm::DbErr` 转成它
    #[derive(Debug)]
    pub struct DbError(pub String);

    impl std::fmt::Display for DbError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for DbError {}
}

impl FlowError {
    /// 业务校验失败
    pub fn biz(msg: impl Into<String>) -> Self {
        FlowError::Biz(msg.into())
    }

    /// 数据不存在
    pub fn not_found(msg: impl Into<String>) -> Self {
        FlowError::NotFound(msg.into())
    }

    /// 断言非空，对应 Java `AssertUtil.isNull(obj, msg)`
    pub fn require_some<T>(value: Option<T>, msg: &str) -> Result<T> {
        value.ok_or_else(|| FlowError::Biz(msg.to_string()))
    }

    /// 断言为真，对应 Java `AssertUtil.isTrue(condition, msg)`
    pub fn require_false(condition: bool, msg: &str) -> Result<()> {
        if condition {
            Err(FlowError::Biz(msg.to_string()))
        } else {
            Ok(())
        }
    }
}

/// 统一返回体，对应 Java `ApiResult<T>`
#[derive(Debug, Clone, Serialize)]
pub struct ApiResult<T> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResult<T> {
    pub fn ok(data: T) -> Self {
        Self { code: 200, msg: "操作成功".to_string(), data: Some(data) }
    }

    pub fn ok_empty() -> Self {
        Self { code: 200, msg: "操作成功".to_string(), data: None }
    }

    pub fn fail(msg: impl Into<String>) -> Self {
        Self { code: 500, msg: msg.into(), data: None }
    }

    /// 指定业务码的失败响应（对应 Java `ApiResult.fail(int, String)`）
    pub fn fail_with(code: i32, msg: impl Into<String>) -> Self {
        Self { code, msg: msg.into(), data: None }
    }
}

impl<T> From<FlowError> for ApiResult<T> {
    fn from(err: FlowError) -> Self {
        ApiResult::fail(err.to_string())
    }
}
