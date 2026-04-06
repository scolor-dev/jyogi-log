use chrono::Utc;
use sqlx::PgPool;

use crate::adapter::persistence::{
    refresh_token_repository, session_repository, user_credential_repository,
    user_identity_repository, user_profile_repository, user_repository,
};
use crate::adapter::security::hashing::{bcrypt_hash, bcrypt_verify, sha256};
use crate::adapter::security::token::{jwt::{self, JwtConfig}, opaque};

use crate::domain::models::session::Session;
use crate::domain::models::token::TokenPair;
use crate::domain::models::user::User;
use crate::error::AppError;

// ─── signup ───────────────────────────────────────────────────────────────────

pub async fn signup(
    pool: &PgPool,
    username: String,
    password: String,
    display_name: String,
) -> Result<User, AppError> {
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
    user_profile_repository::create(&mut tx, user.id, user.uuid, &display_name).await?;

    let activated = user_repository::activate(&mut tx, user.id).await?;

    tx.commit().await.map_err(AppError::Database)?;

    Ok(activated)
}

// ─── login ────────────────────────────────────────────────────────────────────

pub async fn login(
    pool: &PgPool,
    jwt_cfg: &JwtConfig,
    username: String,
    password: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<TokenPair, AppError> {
    let identity = user_identity_repository::find_by_identifier(pool, "username", &username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid username or password".to_string()))?;

    let user = user_repository::find_by_id(pool, identity.user_id)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid username or password".to_string()))?;

    if !user.is_active() {
        return Err(AppError::Unauthorized("account is not active".to_string()));
    }

    let credential = user_credential_repository::find_by_user_and_type(pool, user.id, "password")
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid username or password".to_string()))?;

    if !bcrypt_verify(&password, &credential.secret)? {
        return Err(AppError::Unauthorized("invalid username or password".to_string()));
    }

    let expires_at = Utc::now() + chrono::Duration::days(30);
    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    let session = session_repository::create(
        &mut tx,
        user.id,
        user.uuid,
        ip_address.as_deref(),
        user_agent.as_deref(),
        expires_at,
    )
    .await?;

    let (raw_refresh, refresh_hash) = opaque::generate();
    refresh_token_repository::create(
        &mut tx,
        &refresh_hash,
        session.id,
        user.id,
        user.uuid,
        session.session_uuid,
        expires_at,
    )
    .await?;

    tx.commit().await.map_err(AppError::Database)?;

    let access_token = jwt::generate(jwt_cfg, user.uuid, session.session_uuid)?;

    Ok(TokenPair {
        access_token,
        refresh_token: raw_refresh,
        session_uuid: session.session_uuid,
    })
}

// ─── refresh ──────────────────────────────────────────────────────────────────

pub async fn refresh(
    pool: &PgPool,
    jwt_cfg: &JwtConfig,
    raw_refresh_token: String,
) -> Result<TokenPair, AppError> {
    let token_hash = sha256(&raw_refresh_token);

    let stored = refresh_token_repository::find_by_hash(pool, &token_hash)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid refresh token".to_string()))?;

    if !stored.is_valid() {
        return Err(AppError::Unauthorized(
            "refresh token is expired or already used".to_string(),
        ));
    }

    let session: Session = session_repository::find_by_id(pool, stored.session_id)
        .await?
        .ok_or_else(|| AppError::Unauthorized("session not found".to_string()))?;

    if !session.is_valid() {
        return Err(AppError::Unauthorized(
            "session is expired or revoked".to_string(),
        ));
    }

    let expires_at = Utc::now() + chrono::Duration::days(30);
    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    refresh_token_repository::mark_used(&mut tx, stored.id).await?;

    let (raw_new, new_hash) = opaque::generate();
    refresh_token_repository::create(
        &mut tx,
        &new_hash,
        session.id,
        stored.user_id,
        stored.user_uuid,
        stored.session_uuid,
        expires_at,
    )
    .await?;

    tx.commit().await.map_err(AppError::Database)?;

    session_repository::touch(pool, session.id).await?;

    let access_token = jwt::generate(jwt_cfg, stored.user_uuid, stored.session_uuid)?;

    Ok(TokenPair {
        access_token,
        refresh_token: raw_new,
        session_uuid: stored.session_uuid,
    })
}

// ─── logout ───────────────────────────────────────────────────────────────────

pub async fn logout(pool: &PgPool, session_uuid: uuid::Uuid) -> Result<(), AppError> {
    let session: Session = session_repository::find_by_uuid(pool, session_uuid)
        .await?
        .ok_or_else(|| AppError::NotFound("session not found".to_string()))?;

    refresh_token_repository::revoke_all_by_session(pool, session.id).await?;
    session_repository::revoke(pool, session.id).await?;

    Ok(())
}

// ─── me ───────────────────────────────────────────────────────────────────────

pub async fn me(pool: &PgPool, user_uuid: uuid::Uuid) -> Result<User, AppError> {
    user_repository::find_by_uuid(pool, user_uuid)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))
}