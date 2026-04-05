use sqlx::PgPool;

use crate::{
    adapter::{
        persistence::{user_credential_repository, user_identity_repository, user_repository},
        security::hashing::bcrypt_hash,
    },
    domain::models::user::User,
    error::AppError,
};

pub async fn signup(
    pool: &PgPool,
    username: String,
    password: String,
    _display_name: String, // TODO: UserProfile実装後に保存する
) -> Result<User, AppError> {
    // username 重複チェック（単独読み取りなのでtx外）
    if user_identity_repository::find_by_identifier(pool, "username", &username)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("username already taken".to_string()));
    }

    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    let user = user_repository::create(&mut tx).await?;
    let hashed = bcrypt_hash(&password)?;
    user_credential_repository::create(&mut tx, user.id, "password", &hashed).await?;
    user_identity_repository::create(&mut tx, user.id, "username", &username, true).await?;
    let activated = user_repository::activate(&mut tx, user.id).await?;

    tx.commit().await.map_err(AppError::Database)?;

    Ok(activated)
}