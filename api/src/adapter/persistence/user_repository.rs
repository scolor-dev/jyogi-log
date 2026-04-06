use sqlx::{PgConnection, PgPool};

use crate::domain::models::user::User;
use crate::error::AppError;

pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<User>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, uuid, status::text, created_at, updated_at, deleted_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn find_by_uuid(pool: &PgPool, uuid: uuid::Uuid) -> Result<Option<User>, AppError> {
    sqlx::query_as(
        r#"
        SELECT id, uuid, status::text, created_at, updated_at, deleted_at
        FROM users
        WHERE uuid = $1
        "#,
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(conn: &mut PgConnection) -> Result<User, AppError> {
    sqlx::query_as(
        r#"
        INSERT INTO users DEFAULT VALUES
        RETURNING id, uuid, status::text, created_at, updated_at, deleted_at
        "#,
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn activate(conn: &mut PgConnection, id: i64) -> Result<User, AppError> {
    sqlx::query_as(
        r#"
        UPDATE users
        SET status = 'active'::user_status
        WHERE id = $1
        RETURNING id, uuid, status::text, created_at, updated_at, deleted_at
        "#,
    )
    .bind(id)
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn inactivate(pool: &PgPool, id: i64) -> Result<User, AppError> {
    sqlx::query_as(
        r#"
        UPDATE users
        SET status = 'inactive'::user_status
        WHERE id = $1
        RETURNING id, uuid, status::text, created_at, updated_at, deleted_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn suspend(pool: &PgPool, id: i64) -> Result<User, AppError> {
    sqlx::query_as(
        r#"
        UPDATE users
        SET status = 'suspended'::user_status
        WHERE id = $1
        RETURNING id, uuid, status::text, created_at, updated_at, deleted_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<User, AppError> {
    sqlx::query_as(
        r#"
        UPDATE users
        SET deleted_at = NOW()
        WHERE id = $1
        RETURNING id, uuid, status::text, created_at, updated_at, deleted_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}