use rand::Rng;

use crate::adapter::security::hashing::sha256;

/// ランダムなopaqueトークンを生成してraw文字列とそのSHA-256ハッシュを返す
pub fn generate() -> (String, String) {
    let mut bytes = [0u8; 48];
    rand::rng().fill_bytes(&mut bytes);
    let raw = hex::encode(bytes);
    let hash = sha256(&raw);
    (raw, hash)
}