use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::domain::models::refresh_token::RefreshToken;
use crate::error::AppError;

pub async fn find_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<RefreshToken>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, token_hash, session_id, user_id, user_uuid, session_uuid,
               is_used, expires_at, created_at
        FROM refresh_tokens
        WHERE token_hash = $1
        "#,
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    token_hash: &str,
    session_id: i64,
    user_id: i64,
    user_uuid: Uuid,
    session_uuid: Uuid,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<RefreshToken, AppError> {
    sqlx::query_as(
        r#"
        INSERT INTO refresh_tokens
            (token_hash, session_id, user_id, user_uuid, session_uuid, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, token_hash, session_id, user_id, user_uuid, session_uuid,
                  is_used, expires_at, created_at
        "#,
    )
    .bind(token_hash)
    .bind(session_id)
    .bind(user_id)
    .bind(user_uuid)
    .bind(session_uuid)
    .bind(expires_at)
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn mark_used(
    conn: &mut PgConnection,
    id: i64,
) -> Result<RefreshToken, AppError> {
    sqlx::query_as(
        r#"
        UPDATE refresh_tokens
        SET is_used = true
        WHERE id = $1
        RETURNING id, token_hash, session_id, user_id, user_uuid, session_uuid,
                  is_used, expires_at, created_at
        "#,
    )
    .bind(id)
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn revoke_all_by_session(pool: &PgPool, session_id: i64) -> Result<u64, AppError> {
    sqlx::query(
        r#"
        UPDATE refresh_tokens
        SET is_used = true
        WHERE session_id = $1 AND is_used = false
        "#,
    )
    .bind(session_id)
    .execute(pool)
    .await
    .map(|r| r.rows_affected())
    .map_err(AppError::Database)
}