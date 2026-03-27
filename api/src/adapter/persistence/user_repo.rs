use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::AuthCredential;

/// signup 時のユーザー作成（4テーブルへのトランザクション挿入）
///
/// # Errors
/// - DB エラー時に `sqlx::Error` を返す
/// - identifier 重複時は `PostgreSQL` error code 23505 が返る
pub async fn create_user_txn(
    pool: &PgPool,
    identifier: &str,
    password_hash: &str,
    display_name: &str,
) -> Result<Uuid, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let user_uuid = Uuid::new_v4();
    let normalized = identifier.to_lowercase();

    // 1. users テーブルへ挿入
    let user_id: i64 = sqlx::query_scalar(
        "INSERT INTO users (uuid, status) VALUES ($1, 'active') RETURNING id",
    )
    .bind(user_uuid)
    .fetch_one(&mut *tx)
    .await?;

    // 2. user_profile へ挿入
    sqlx::query("INSERT INTO user_profile (user_id, display_name) VALUES ($1, $2)")
        .bind(user_id)
        .bind(display_name)
        .execute(&mut *tx)
        .await?;

    // 3. user_identities へ挿入
    sqlx::query(
        "INSERT INTO user_identities (user_id, type, identifier, normalized_identifier, is_primary) \
         VALUES ($1, 'username', $2, $3, true)",
    )
    .bind(user_id)
    .bind(identifier)
    .bind(&normalized)
    .execute(&mut *tx)
    .await?;

    // 4. user_credentials へ挿入
    sqlx::query(
        "INSERT INTO user_credentials (user_id, type, secret_hash, is_primary) \
         VALUES ($1, 'password', $2, true)",
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(user_uuid)
}

/// identifier からユーザーの認証情報を取得（login 用）
///
/// # Errors
/// - DB エラー時に `sqlx::Error` を返す
pub async fn find_credential_by_identifier(
    pool: &PgPool,
    identifier: &str,
) -> Result<Option<AuthCredential>, sqlx::Error> {
    let normalized = identifier.to_lowercase();

    sqlx::query_as::<_, AuthCredential>(
        "SELECT uc.user_id, uc.secret_hash \
         FROM user_credentials uc \
         JOIN user_identities ui ON ui.user_id = uc.user_id \
         WHERE ui.normalized_identifier = $1 \
           AND ui.revoked_at IS NULL \
           AND uc.revoked_at IS NULL \
           AND uc.type = 'password'",
    )
    .bind(&normalized)
    .fetch_optional(pool)
    .await
}
