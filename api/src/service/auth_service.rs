use std::{net::IpAddr, sync::Arc};

use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    config::JwtConfig,
    domain::{
        models::user_credential::CredentialType,
        models::user_identity::IdentityType,
        ports::{
            refresh_token_repository::RefreshTokenRepository,
            session_repository::SessionRepository,
            user_credential_repository::UserCredentialRepository,
            user_identity_repository::UserIdentityRepository,
            user_repository::UserRepository,
        },
    },
    error::AppError,
};

// ── JWT Claims ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// user.uuid (外部公開用)
    pub sub: String,
    /// session.uuid
    pub sid: String,
    pub iat: i64,
    pub exp: i64,
}

// ── DTOs ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

// ── AuthService ───────────────────────────────────────────────────────────────

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    identity_repo: Arc<dyn UserIdentityRepository>,
    credential_repo: Arc<dyn UserCredentialRepository>,
    session_repo: Arc<dyn SessionRepository>,
    refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    jwt_config: JwtConfig,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        identity_repo: Arc<dyn UserIdentityRepository>,
        credential_repo: Arc<dyn UserCredentialRepository>,
        session_repo: Arc<dyn SessionRepository>,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        jwt_config: JwtConfig,
    ) -> Self {
        Self {
            user_repo,
            identity_repo,
            credential_repo,
            session_repo,
            refresh_token_repo,
            jwt_config,
        }
    }

    // ── Signup ────────────────────────────────────────────────────────────────

    pub async fn signup(&self, req: SignupRequest) -> Result<AuthResponse, AppError> {
        validate_username(&req.username)?;
        validate_password(&req.password)?;

        // username 重複チェック
        let existing = self
            .identity_repo
            .find_by_identifier(&IdentityType::Username, &req.username)
            .await?;
        if existing.is_some() {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        // 1. users レコード作成（status: pending → active）
        let user = self.user_repo.create().await?;
        let user = self.user_repo.activate(user.id).await?;

        // 2. user_identities にユーザー名を登録
        self.identity_repo
            .create(user.id, IdentityType::Username, &req.username, true)
            .await?;

        // 3. user_credentials にパスワードハッシュを登録
        let hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        self.credential_repo
            .create(user.id, CredentialType::Password, &hash)
            .await?;

        // 4. セッション発行（signup 時は IP/UA なし）
        self.issue_token_pair(user.id, user.uuid, None, None).await
    }

    // ── Login ─────────────────────────────────────────────────────────────────

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        // 1. identity 検索
        let identity = self
            .identity_repo
            .find_by_identifier(&IdentityType::Username, &req.username)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // 2. user 取得・ステータス確認
        let user = self
            .user_repo
            .find_by_id(identity.user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if !user.is_active() {
            return Err(AppError::Unauthorized("Account is not active".to_string()));
        }

        // 3. パスワード検証
        let credential = self
            .credential_repo
            .find_by_user_and_type(user.id, &CredentialType::Password)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        let valid = bcrypt::verify(&req.password, &credential.secret)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        if !valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // 4. セッション発行
        self.issue_token_pair(user.id, user.uuid, req.ip_address, req.user_agent)
            .await
    }

    // ── Refresh ───────────────────────────────────────────────────────────────

    pub async fn refresh(&self, req: RefreshRequest) -> Result<AuthResponse, AppError> {
        let hash = sha256_hex(&req.refresh_token);

        let token = self
            .refresh_token_repo
            .find_by_hash(&hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        if !token.is_valid() {
            // 使用済み or 期限切れ → セッションごと無効化（Token Reuse 検知）
            self.refresh_token_repo
                .revoke_all_by_session(token.session_id)
                .await?;
            self.session_repo.revoke(token.session_id).await?;
            return Err(AppError::Unauthorized("Refresh token is invalid".to_string()));
        }

        // セッション確認
        let session = self
            .session_repo
            .find_by_id(token.session_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Session not found".to_string()))?;

        if !session.is_valid() {
            return Err(AppError::Unauthorized("Session has expired".to_string()));
        }

        // Rotate: 旧トークンを使用済みにしてから新ペアを発行
        self.refresh_token_repo.mark_used(&hash).await?;
        self.session_repo.touch(session.id).await?;

        let (raw_refresh, refresh_hash) = generate_refresh_token();
        self.refresh_token_repo
            .create(
                session.id,
                token.user_id,
                token.user_uuid,
                session.session_uuid,
                &refresh_hash,
            )
            .await?;

        let access_token = self.generate_access_token(token.user_uuid, session.session_uuid)?;

        Ok(AuthResponse {
            access_token,
            refresh_token: raw_refresh,
        })
    }

    // ── Logout ────────────────────────────────────────────────────────────────

    pub async fn logout(&self, req: LogoutRequest) -> Result<(), AppError> {
        let hash = sha256_hex(&req.refresh_token);

        let token = self
            .refresh_token_repo
            .find_by_hash(&hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        // セッション単位で全トークン無効化 + セッション revoke
        self.refresh_token_repo
            .revoke_all_by_session(token.session_id)
            .await?;
        self.session_repo.revoke(token.session_id).await?;

        Ok(())
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    async fn issue_token_pair(
        &self,
        user_id: i64,
        user_uuid: uuid::Uuid,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> Result<AuthResponse, AppError> {
        let session = self
            .session_repo
            .create(user_id, user_uuid, ip_address, user_agent)
            .await?;

        let (raw_refresh, refresh_hash) = generate_refresh_token();
        self.refresh_token_repo
            .create(
                session.id,
                user_id,
                user_uuid,
                session.session_uuid,
                &refresh_hash,
            )
            .await?;

        let access_token = self.generate_access_token(user_uuid, session.session_uuid)?;

        Ok(AuthResponse {
            access_token,
            refresh_token: raw_refresh,
        })
    }

    fn generate_access_token(
        &self,
        user_uuid: uuid::Uuid,
        session_uuid: uuid::Uuid,
    ) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_uuid.to_string(),
            sid: session_uuid.to_string(),
            iat: now,
            exp: now + self.jwt_config.expires_in_secs,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
    }
}

// ── Pure functions ────────────────────────────────────────────────────────────

fn generate_refresh_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let raw = hex::encode(bytes);
    let hash = sha256_hex(&raw);
    (raw, hash)
}

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn validate_username(username: &str) -> Result<(), AppError> {
    if username.len() < 3 || username.len() > 32 {
        return Err(AppError::BadRequest(
            "Username must be between 3 and 32 characters".to_string(),
        ));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(AppError::BadRequest(
            "Username must contain only letters, numbers, or underscores".to_string(),
        ));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    Ok(())
}