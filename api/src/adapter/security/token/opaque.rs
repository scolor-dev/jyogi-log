use rand::RngCore;

use crate::adapter::security::hashing::sha256;

/// 32バイトのランダムトークンを生成し `(raw, hash)` を返す。
/// raw  … クライアントに渡す生トークン（hex文字列）
/// hash … DBに保存するSHA256ハッシュ（hex文字列）
pub fn generate() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let raw = hex::encode(bytes);
    let hash = sha256(raw.as_str());
    (raw, hash)
}