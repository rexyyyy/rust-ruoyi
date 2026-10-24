//! 统一错误类型。
//!
//! 旧版本基于 rbatis/redis，错误枚举里塞进了大量 ORM 与缓存实现的 `From`，
//! 现在 ORM 换成 sea-orm、也不再直接碰 redis，这里只保留真正需要的转换。

use std::fmt::{self, Debug, Display};

/// 库内统一 Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    E(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::E(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        Error::E(arg.to_string())
    }
}

impl From<String> for Error {
    fn from(arg: String) -> Self {
        Error::E(arg)
    }
}

impl From<std::io::Error> for Error {
    fn from(arg: std::io::Error) -> Self {
        Error::E(arg.to_string())
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(arg: sea_orm::DbErr) -> Self {
        Error::E(arg.to_string())
    }
}

impl From<tera::Error> for Error {
    /// tera::Error 的 Display 只有顶层一句（如 `Failed to render 'xxx'`），
    /// 真正原因（变量缺失、过滤器错误等）在 `source()` 链里。
    /// 不展开这条链，线上就只能看到一句废话，排查全靠猜。
    fn from(arg: tera::Error) -> Self {
        let mut msg = arg.to_string();
        let mut src = std::error::Error::source(&arg);
        while let Some(s) = src {
            msg.push_str(&format!(" | caused by: {}", s));
            src = s.source();
        }
        Error::E(msg)
    }
}

impl From<serde_json::Error> for Error {
    fn from(arg: serde_json::Error) -> Self {
        Error::E(arg.to_string())
    }
}
