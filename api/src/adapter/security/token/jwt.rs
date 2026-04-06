use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in_secs: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user_uuid
    pub sid: String,   // session_uuid
    pub iat: i64,
    pub exp: i64,
}

pub fn generate(cfg: &JwtConfig, user_uuid: Uuid, session_uuid: Uuid) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_uuid.to_string(),
        sid: session_uuid.to_string(),
        iat: now,
        exp: now + cfg.expires_in_secs,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.secret.as_bytes()),
    )
    .map_err(|e: jsonwebtoken::errors::Error| AppError::Internal(anyhow::anyhow!("jwt encode error: {}", e)))
}

pub fn verify(cfg: &JwtConfig, token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("invalid token: {}", e)))
}