use hex::encode as hex_encode;
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{
    adapter::persistence::{session_repo, user_repo},
    domain::models::{LoginRequest, MeResponse, SignupRequest, SignupResponse, UserRow},
    error::ApiError,
};

/// セッションクッキー名
pub const SESSION_COOKIE_NAME: &str = "session_token";

/// 32バイトのランダムトークンを生成して hex 文字列で返す
fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex_encode(bytes)
}

/// 生トークンを SHA-256 でハッシュ化して hex 文字列で返す
fn hash_token(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex_encode(digest)
}

/// ユーザー登録処理
///
/// # Errors
/// - identifier 重複時: `ApiError::Conflict`
/// - DB エラー時: `ApiError::InternalServerError`
/// - bcrypt エラー時: `ApiError::InternalServerError`
pub async fn signup(pool: &PgPool, req: &SignupRequest) -> Result<SignupResponse, ApiError> {
    if req.identifier.is_empty() || req.password.is_empty() || req.display_name.is_empty() {
        return Err(ApiError::BadRequest);
    }

    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)?;

    let user_uuid = user_repo::create_user_txn(
        pool,
        &req.identifier,
        &password_hash,
        &req.display_name,
    )
    .await?;

    Ok(SignupResponse { user_id: user_uuid })
}

/// identifier が存在しない場合もダミーハッシュで bcrypt を実行しタイミング差を排除する
const DUMMY_HASH: &str = "$2b$12$GhvMmNVjRW29ulnudl.LDuINE2ZZm5T9VDJ/b5VLg.Ggt.kp7/LDi";

/// ログイン処理。成功時に生セッショントークンを返す
///
/// # Errors
/// - identifier / password 不一致: `ApiError::Unauthorized`
/// - DB エラー時: `ApiError::InternalServerError`
pub async fn login(pool: &PgPool, req: &LoginRequest) -> Result<String, ApiError> {
    let credential = user_repo::find_credential_by_identifier(pool, &req.identifier).await?;

    let (secret_hash, user_found) =
        credential.as_ref().map_or((DUMMY_HASH, false), |c| (c.secret_hash.as_str(), true));

    let valid = match bcrypt::verify(&req.password, secret_hash) {
        Ok(v) => v,
        Err(e) => {
            if user_found {
                return Err(ApiError::from(e));
            }
            false
        }
    };

    if !user_found || !valid {
        tracing::warn!(identifier = %req.identifier, "login failed");
        return Err(ApiError::Unauthorized);
    }

    let raw_token = generate_token();
    let token_hash = hash_token(&raw_token);

    // user_found が true = credential は Some であることが保証されている
    let user_id = credential
        .ok_or(ApiError::Unauthorized)?
        .user_id;
    session_repo::create(pool, user_id, &token_hash).await?;

    Ok(raw_token)
}

/// ログアウト処理
///
/// # Errors
/// - セッションが存在しない / 無効: `ApiError::Unauthorized`
/// - DB エラー時: `ApiError::InternalServerError`
pub async fn logout(pool: &PgPool, raw_token: &str) -> Result<(), ApiError> {
    let token_hash = hash_token(raw_token);
    let session = session_repo::find_valid_by_hash(pool, &token_hash)
        .await?
        .ok_or(ApiError::Unauthorized)?;

    let _ = session.id; // セッション存在確認済み
    session_repo::revoke_by_hash(pool, &token_hash).await?;

    Ok(())
}

/// 現在のセッションに紐づくユーザー情報を返す
///
/// # Errors
/// - セッションが存在しない / 無効: `ApiError::Unauthorized`
/// - DB エラー時: `ApiError::InternalServerError`
pub async fn me(pool: &PgPool, raw_token: &str) -> Result<MeResponse, ApiError> {
    let token_hash = hash_token(raw_token);
    let session = session_repo::find_valid_by_hash(pool, &token_hash)
        .await?
        .ok_or(ApiError::Unauthorized)?;

    let user: UserRow = sqlx::query_as(
        "SELECT u.uuid, up.display_name, ui.identifier \
         FROM users u \
         JOIN user_profile up ON up.user_id = u.id \
         JOIN user_identities ui ON ui.user_id = u.id \
         WHERE u.id = $1 \
           AND ui.revoked_at IS NULL \
           AND ui.is_primary = true \
         LIMIT 1",
    )
    .bind(session.user_id)
    .fetch_one(pool)
    .await
    .map_err(ApiError::from)?;

    Ok(MeResponse {
        user_id: user.uuid,
        display_name: user.display_name,
        identifier: user.identifier,
    })
}

