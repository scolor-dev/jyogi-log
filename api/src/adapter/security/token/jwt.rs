use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::JwtConfig, error::AppError};

// ── Claims ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// user.uuid（外部公開用）
    pub sub: String,
    /// session.uuid
    pub sid: String,
    pub iat: i64,
    pub exp: i64,
}

// ── 生成 ──────────────────────────────────────────────────────────────────────

pub fn generate(
    user_uuid: Uuid,
    session_uuid: Uuid,
    config: &JwtConfig,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_uuid.to_string(),
        sid: session_uuid.to_string(),
        iat: now,
        exp: now + config.expires_in_secs,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
}

// ── 検証 ──────────────────────────────────────────────────────────────────────

pub fn verify(token: &str, config: &JwtConfig) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))
}