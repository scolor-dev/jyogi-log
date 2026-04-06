use sha2::{Digest, Sha256};

use crate::error::AppError;

/// 任意の文字列をSHA256でハッシュしhex文字列で返す。
pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

/// パスワードをbcryptでハッシュする。
pub fn bcrypt_hash(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
}

/// パスワードとbcryptハッシュを検証する。
pub fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
}