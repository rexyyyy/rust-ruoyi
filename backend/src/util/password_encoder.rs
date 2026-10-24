use bcrypt::{hash, verify, DEFAULT_COST};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub struct PasswordEncoder {}

impl PasswordEncoder {
    /// 使用 bcrypt（自带随机盐 + 可调工作因子）对新密码编码。
    /// 编码失败（极端情况）回退到历史 SHA3-256，保证不会 panic。
    pub fn encode(raw_password: &str) -> String {
        match hash(raw_password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => legacy_sha3(raw_password),
        }
    }

    /// 校验密码：
    /// - 库中为 bcrypt 哈希（`$2a$/$2b$/$2y$` 前缀）-> bcrypt 校验；
    /// - 否则按历史 SHA3-256 处理（兼容存量用户，登录后可迁移为新哈希）。
    pub fn verify(password: &str, raw_password: &str) -> bool {
        if is_bcrypt_hash(password) {
            verify(raw_password, password).unwrap_or(false)
        } else {
            password == legacy_sha3(raw_password)
        }
    }
}

/// 判断是否 bcrypt 哈希（便于新旧兼容）
fn is_bcrypt_hash(hash: &str) -> bool {
    hash.starts_with("$2a$") || hash.starts_with("$2b$") || hash.starts_with("$2y$")
}

/// 历史编码：无盐 SHA3-256（仅用于校验存量数据）
fn legacy_sha3(raw_password: &str) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(raw_password);
    hasher.result_str().to_lowercase()
}

#[cfg(test)]
mod test {
    use crate::util::password_encoder::PasswordEncoder;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("AsAPig!");
        println!("{}", s);
        // bcrypt 带随机盐，同一密码两次编码结果不同
        assert_ne!(
            PasswordEncoder::encode("AsAPig!"),
            PasswordEncoder::encode("AsAPig!")
        );
    }

    #[test]
    fn test_verify_bcrypt() {
        let encoded = PasswordEncoder::encode("AsAPig!");
        assert!(PasswordEncoder::verify(&encoded, "AsAPig!"));
        assert!(!PasswordEncoder::verify(&encoded, "wrong"));
    }

    #[test]
    fn test_verify_legacy_sha3() {
        // 历史哈希仍可校验（迁移兼容）
        let mut hasher = crypto::sha3::Sha3::sha3_256();
        crypto::digest::Digest::input_str(&mut hasher, "12345");
        let legacy = crypto::digest::Digest::result_str(&mut hasher).to_lowercase();
        assert!(PasswordEncoder::verify(&legacy, "12345"));
    }
}
