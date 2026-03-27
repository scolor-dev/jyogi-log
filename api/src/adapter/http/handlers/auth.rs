use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};

use crate::{
    domain::models::{LoginRequest, SignupRequest},
    error::ApiError,
    service::auth::{self, SESSION_COOKIE_NAME},
    state::AppState,
};

/// POST /auth/signup — ユーザー登録
///
/// # Errors
/// - バリデーション失敗: 400
/// - identifier 重複: 409
/// - その他: 500
pub async fn signup(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> Result<(StatusCode, Json<crate::domain::models::SignupResponse>), ApiError> {
    let response = auth::signup(state.db(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// POST /auth/login — ログイン
///
/// # Errors
/// - 認証失敗: 401
/// - その他: 500
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, StatusCode), ApiError> {
    let raw_token = auth::login(state.db(), &body).await?;

    let cookie = Cookie::build((SESSION_COOKIE_NAME, raw_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    Ok((jar.add(cookie), StatusCode::OK))
}

/// POST /auth/logout — ログアウト
///
/// # Errors
/// - セッション無効: 401
/// - その他: 500
pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    let raw_token = jar
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_owned())
        .ok_or(ApiError::Unauthorized)?;

    auth::logout(state.db(), &raw_token).await?;

    let removal = Cookie::build((SESSION_COOKIE_NAME, ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();

    Ok((jar.remove(removal), StatusCode::NO_CONTENT))
}

/// GET /auth/me — 現在のユーザー情報取得
///
/// # Errors
/// - セッション無効: 401
/// - その他: 500
pub async fn me(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<crate::domain::models::MeResponse>, ApiError> {
    let raw_token = jar
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_owned())
        .ok_or(ApiError::Unauthorized)?;

    let response = auth::me(state.db(), &raw_token).await?;
    Ok(Json(response))
}
