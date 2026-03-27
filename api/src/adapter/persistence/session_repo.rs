use sqlx::PgPool;

use crate::domain::models::Session;

/// セッションを作成する（login 用）
///
/// # Errors
/// - DB エラー時に `sqlx::Error` を返す
pub async fn create(
    pool: &PgPool,
    user_id: i64,
    token_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions (user_id, token_hash, expires_at) \
         VALUES ($1, $2, NOW() + INTERVAL '30 days')",
    )
    .bind(user_id)
    .bind(token_hash)
    .execute(pool)
    .await?;

    Ok(())
}

/// 有効なセッションをトークンハッシュで検索する（logout / me 用）
///
/// # Errors
/// - DB エラー時に `sqlx::Error` を返す
pub async fn find_valid_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        "SELECT id, user_id \
         FROM sessions \
         WHERE token_hash = $1 \
           AND revoked_at IS NULL \
           AND expires_at > NOW()",
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
}

/// セッションを失効させる（logout 用）
///
/// # Errors
/// - DB エラー時に `sqlx::Error` を返す
pub async fn revoke_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE sessions SET revoked_at = NOW() WHERE token_hash = $1 AND revoked_at IS NULL",
    )
    .bind(token_hash)
    .execute(pool)
    .await?;

    Ok(())
}
