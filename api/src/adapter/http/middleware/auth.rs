use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    adapter::security::token::jwt,
    config::JwtConfig,
    error::AppError,
};

// ── 認証済みユーザー情報 ──────────────────────────────────────────────────────

/// JWT検証後に後続のhandlerへ渡す認証済み情報。
/// `Extension<AuthUser>` として取り出す。
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_uuid: Uuid,
    pub session_uuid: Uuid,
}

// ── ミドルウェア ──────────────────────────────────────────────────────────────

/// `Authorization: Bearer <token>` を検証して `AuthUser` をExtensionに追加する。
/// 検証失敗時は `401` を返してhandlerに到達させない。
pub async fn require_auth(
    State(config): State<JwtConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_bearer(req.headers())?;
    let claims = jwt::verify(token, &config)?;

    let user_uuid = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid token subject".to_string()))?;
    let session_uuid = Uuid::parse_str(&claims.sid)
        .map_err(|_| AppError::Unauthorized("Invalid token session".to_string()))?;

    req.extensions_mut().insert(AuthUser { user_uuid, session_uuid });

    Ok(next.run(req).await)
}

// ── helper ────────────────────────────────────────────────────────────────────

fn extract_bearer(headers: &axum::http::HeaderMap) -> Result<&str, AppError> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

    value
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Authorization header must be Bearer".to_string()))
}