use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::adapter::http::validator::{validate_display_name, validate_password, validate_username};
use crate::adapter::security::token::jwt;
use crate::service::auth_service;
use crate::state::AppState;

// ─── Request / Response ───────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: &'static str,
}

// ─── Handlers ─────────────────────────────────────────────────────────────────

pub async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> impl IntoResponse {
    if let Some(message) = validate_username(&req.username)
        .or_else(|| validate_password(&req.password))
        .or_else(|| validate_display_name(&req.display_name))
    {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { message })).into_response();
    }

    match auth_service::signup(&state.pool, req.username, req.password, req.display_name).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            tracing::error!("signup error: {:?}", e);
            e.into_response()
        }
    }
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    let ip_address = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match auth_service::login(&state.pool, &state.jwt, req.username, req.password, ip_address, user_agent).await {
        Ok(token_pair) => {
            let cookie = format!(
                "refresh_token={}; HttpOnly; SameSite=Strict; Path=/auth/refresh; Max-Age=2592000",
                token_pair.refresh_token
            );
            (
                StatusCode::OK,
                [(header::SET_COOKIE, cookie)],
                Json(LoginResponse { access_token: token_pair.access_token }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("login error: {:?}", e);
            e.into_response()
        }
    }
}

pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let raw_refresh_token = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|c| {
                let c = c.trim();
                c.strip_prefix("refresh_token=").map(|v| v.to_string())
            })
        });

    let raw_refresh_token = match raw_refresh_token {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "missing refresh token"}))).into_response(),
    };

    match auth_service::refresh(&state.pool, &state.jwt, raw_refresh_token).await {
        Ok(token_pair) => {
            let cookie = format!(
                "refresh_token={}; HttpOnly; SameSite=Strict; Path=/auth/refresh; Max-Age=2592000",
                token_pair.refresh_token
            );
            (
                StatusCode::OK,
                [(header::SET_COOKIE, cookie)],
                Json(LoginResponse { access_token: token_pair.access_token }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("refresh error: {:?}", e);
            e.into_response()
        }
    }
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let bearer = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer ").map(|s| s.to_string()));

    let token = match bearer {
        Some(t) => t,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let claims = match jwt::verify(&state.jwt, &token) {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let session_uuid = match claims.sid.parse::<uuid::Uuid>() {
        Ok(u) => u,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let clear_cookie = "refresh_token=; HttpOnly; SameSite=Strict; Path=/auth/refresh; Max-Age=0";

    match auth_service::logout(&state.pool, session_uuid).await {
        Ok(_) => (
            StatusCode::OK,
            [(header::SET_COOKIE, clear_cookie)],
        ).into_response(),
        Err(e) => {
            tracing::error!("logout error: {:?}", e);
            e.into_response()
        }
    }
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let bearer = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer ").map(|s| s.to_string()));

    let token = match bearer {
        Some(t) => t,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let claims = match jwt::verify(&state.jwt, &token) {
        Ok(c) => c,
        Err(e) => return e.into_response(),
    };

    let user_uuid = match claims.sub.parse::<uuid::Uuid>() {
        Ok(u) => u,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match auth_service::me(&state.pool, user_uuid).await {
        Ok(user) => Json(serde_json::json!({
            "uuid": user.uuid,
            "status": user.status,
            "created_at": user.created_at,
        })).into_response(),
        Err(e) => {
            tracing::error!("me error: {:?}", e);
            e.into_response()
        }
    }
}