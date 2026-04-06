use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::domain::models::session::Session;
use crate::error::AppError;

pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Session>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, session_uuid, user_id, user_uuid,
               host(ip_address) AS ip_address, user_agent,
               last_active_at, expires_at, revoked_at, created_at, updated_at
        FROM sessions
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn find_by_uuid(pool: &PgPool, session_uuid: Uuid) -> Result<Option<Session>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, session_uuid, user_id, user_uuid,
               host(ip_address) AS ip_address, user_agent,
               last_active_at, expires_at, revoked_at, created_at, updated_at
        FROM sessions
        WHERE session_uuid = $1
        "#,
    )
    .bind(session_uuid)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    user_id: i64,
    user_uuid: Uuid,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<Session, AppError> {
    sqlx::query_as(
        r#"
        INSERT INTO sessions (user_id, user_uuid, ip_address, user_agent, expires_at)
        VALUES ($1, $2, $3::inet, $4, $5)
        RETURNING id, session_uuid, user_id, user_uuid,
                  host(ip_address) AS ip_address, user_agent,
                  last_active_at, expires_at, revoked_at, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(user_uuid)
    .bind(ip_address)
    .bind(user_agent)
    .bind(expires_at)
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn touch(pool: &PgPool, id: i64) -> Result<Session, AppError> {
    sqlx::query_as(
        r#"
        UPDATE sessions
        SET last_active_at = NOW()
        WHERE id = $1
        RETURNING id, session_uuid, user_id, user_uuid,
                  host(ip_address) AS ip_address, user_agent,
                  last_active_at, expires_at, revoked_at, created_at, updated_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn revoke(pool: &PgPool, id: i64) -> Result<Session, AppError> {
    sqlx::query_as(
        r#"
        UPDATE sessions
        SET revoked_at = NOW()
        WHERE id = $1
        RETURNING id, session_uuid, user_id, user_uuid,
                  host(ip_address) AS ip_address, user_agent,
                  last_active_at, expires_at, revoked_at, created_at, updated_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn revoke_all_by_user(pool: &PgPool, user_id: i64) -> Result<u64, AppError> {
    sqlx::query(
        r#"
        UPDATE sessions
        SET revoked_at = NOW()
        WHERE user_id = $1 AND revoked_at IS NULL
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await
    .map(|r| r.rows_affected())
    .map_err(AppError::Database)
}