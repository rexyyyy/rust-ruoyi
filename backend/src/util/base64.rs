//! base64 编解码 —— 统一走 `base64` crate 的 STANDARD engine（RFC 4648，带 '=' 填充）。
//!
//! 历史遗留说明：本模块曾有一份自研实现，其尾部 remainder 下标计算错误（循环变量 shadowing），
//! 导致输入长度不是 3 的倍数时末尾 1~2 字节被丢弃且不补 '='，
//! base64 串"格式合法"但数据被截断 —— 曾造成验证码 PNG 渲染为破碎图。
//! 自研实现已废弃，改为标准实现的薄封装，**请勿再手写 base64**。

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

/// 标准 base64 编码（带 '=' 填充，无换行）
pub fn encode(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

/// 标准 base64 解码
#[allow(dead_code)] // 当前仅验证码用到编码侧；保留解码侧供后续接口使用
pub fn decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    STANDARD.decode(input)
}

#[cfg(test)]
mod test {
    use super::{decode, encode};

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(&[]), "");
    }

    /// 长度不是 3 的倍数时，尾部必须正确编码并补 '=' —— 自研实现正是在这里出错
    #[test]
    fn test_encode_remainder() {
        assert_eq!(encode(b"M"), "TQ=="); // remain = 1
        assert_eq!(encode(b"Ma"), "TWE="); // remain = 2
        assert_eq!(encode(b"Man"), "TWFu"); // remain = 0
        assert_eq!(encode(b"Many"), "TWFueQ==");
    }

    /// 对齐 RFC 4648 标准测试向量
    #[test]
    fn test_encode_rfc_vectors() {
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
        assert_eq!(encode(b"foo"), "Zm9v");
        assert_eq!(encode(b"foob"), "Zm9vYg==");
        assert_eq!(encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(encode(b"foobar"), "Zm9vYmFy");
    }

    /// 全字节值覆盖，并与 Python/通用实现一致的 round-trip
    #[test]
    fn test_round_trip_all_bytes() {
        let all: Vec<u8> = (0..=255u8).collect();
        let encoded = encode(&all);
        assert_eq!(encoded.len() % 4, 0, "base64 长度必须是 4 的倍数");
        assert_eq!(decode(&encoded).unwrap(), all, "round-trip 必须还原原始字节");
    }

    /// 长度 0..=64 全量 round-trip，覆盖 remain = 0 / 1 / 2 三种情况
    #[test]
    fn test_round_trip_all_lengths() {
        for len in 0..=64usize {
            let data: Vec<u8> = (0..len).map(|i| (i * 37 + 11) as u8).collect();
            let encoded = encode(&data);
            assert_eq!(encoded.len() % 4, 0, "len={len} 时 base64 长度非 4 的倍数");
            assert_eq!(
                decode(&encoded).unwrap(),
                data,
                "len={len} 时 round-trip 未还原原始字节"
            );
        }
    }
}
