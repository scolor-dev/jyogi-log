use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::{
    domain::models::user::User,
    error::AppError,
};

pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<User>, AppError> {
    sqlx::query_as!(
        User,
        "SELECT id, uuid, status, created_at, updated_at, deleted_at FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn find_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<User>, AppError> {
    sqlx::query_as!(
        User,
        "SELECT id, uuid, status, created_at, updated_at, deleted_at FROM users WHERE uuid = $1",
        uuid
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(conn: &mut PgConnection) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (uuid, status) VALUES (gen_random_uuid(), 'pending')
         RETURNING id, uuid, status, created_at, updated_at, deleted_at"
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn activate(conn: &mut PgConnection, id: i64) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        "UPDATE users SET status = 'active', updated_at = now() WHERE id = $1
         RETURNING id, uuid, status, created_at, updated_at, deleted_at",
        id
    )
    .fetch_one(conn)
    .await
    .map_err(AppError::Database)
}

pub async fn inactivate(pool: &PgPool, id: i64) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        "UPDATE users SET status = 'inactive', updated_at = now() WHERE id = $1
         RETURNING id, uuid, status, created_at, updated_at, deleted_at",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn suspend(pool: &PgPool, id: i64) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        "UPDATE users SET status = 'suspended', updated_at = now() WHERE id = $1
         RETURNING id, uuid, status, created_at, updated_at, deleted_at",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        "UPDATE users SET deleted_at = now(), updated_at = now() WHERE id = $1
         RETURNING id, uuid, status, created_at, updated_at, deleted_at",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}