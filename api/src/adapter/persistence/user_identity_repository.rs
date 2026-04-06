use sqlx::{PgConnection, PgPool};

use crate::domain::models::user_identity::UserIdentity;
use crate::error::AppError;

pub async fn find_by_identifier(
    pool: &PgPool,
    identity_type: &str,
    identifier: &str,
) -> Result<Option<UserIdentity>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, user_id, "type"::text AS identity_type, identifier, is_primary, created_at, updated_at
        FROM user_identities
        WHERE "type" = $1::identity_type AND identifier = $2
        "#,
    )
    .bind(identity_type)
    .bind(identifier)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    user_id: i64,
    identity_type: &str,
    identifier: &str,
    is_primary: bool,
) -> Result<UserIdentity, AppError> {
    sqlx::query_as(
        r#"
        INSERT INTO user_identities (user_id, "type", identifier, is_primary)
        VALUES ($1, $2::identity_type, $3, $4)
        RETURNING id, user_id, "type"::text AS identity_type, identifier, is_primary, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(identity_type)
    .bind(identifier)
    .bind(is_primary)
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}