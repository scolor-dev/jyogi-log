use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// POST /auth/signup リクエストボディ
#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub identifier: String,
    pub password: String,
    pub display_name: String,
}

/// POST /auth/login リクエストボディ
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

/// POST /auth/signup レスポンス
#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub user_id: Uuid,
}

/// GET /auth/me レスポンス
#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user_id: Uuid,
    pub display_name: String,
    pub identifier: String,
}

/// DB から取得したセッション情報
#[derive(Debug, sqlx::FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
}

/// DB から取得した認証用情報（login 照合用）
#[derive(Debug, sqlx::FromRow)]
pub struct AuthCredential {
    pub user_id: i64,
    pub secret_hash: String,
}

/// DB から取得したユーザー情報（/me 用）
#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub uuid: Uuid,
    pub display_name: String,
    pub identifier: String,
}
