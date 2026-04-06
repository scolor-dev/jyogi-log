use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::domain::models::user_profile::UserProfile;
use crate::error::AppError;

pub async fn find_by_user_id(
    pool: &PgPool,
    user_id: i64,
) -> Result<Option<UserProfile>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, user_id, user_uuid, display_name, tagline, bio, avatar_url, created_at, updated_at
        FROM user_profiles
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn find_by_user_uuid(
    pool: &PgPool,
    user_uuid: Uuid,
) -> Result<Option<UserProfile>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, user_id, user_uuid, display_name, tagline, bio, avatar_url, created_at, updated_at
        FROM user_profiles
        WHERE user_uuid = $1
        "#,
    )
    .bind(user_uuid)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    conn: &mut PgConnection,
    user_id: i64,
    user_uuid: Uuid,
    display_name: &str,
) -> Result<UserProfile, AppError> {
    sqlx::query_as(
        r#"
        INSERT INTO user_profiles (user_id, user_uuid, display_name)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, user_uuid, display_name, tagline, bio, avatar_url, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(user_uuid)
    .bind(display_name)
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn update(
    pool: &PgPool,
    user_id: i64,
    display_name: Option<&str>,
    tagline: Option<&str>,
    bio: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<UserProfile, AppError> {
    sqlx::query_as(
        r#"
        UPDATE user_profiles
        SET
            display_name = COALESCE($2, display_name),
            tagline      = COALESCE($3, tagline),
            bio          = COALESCE($4, bio),
            avatar_url   = COALESCE($5, avatar_url)
        WHERE user_id = $1
        RETURNING id, user_id, user_uuid, display_name, tagline, bio, avatar_url, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(display_name)
    .bind(tagline)
    .bind(bio)
    .bind(avatar_url)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}