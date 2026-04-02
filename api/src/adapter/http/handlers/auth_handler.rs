use axum::{
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::{net::SocketAddr, sync::Arc};

use crate::{
    error::AppError,
    service::auth_service::{
        AuthService, LoginRequest, LogoutRequest, RefreshRequest, SignupRequest,
    },
};

// ── Signup ────────────────────────────────────────────────────────────────────

pub async fn signup(
    State(auth_svc): State<Arc<AuthService>>,
    Json(body): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    let resp = auth_svc.signup(body).await?;
    Ok((StatusCode::CREATED, Json(resp)))
}

// ── Login ─────────────────────────────────────────────────────────────────────

pub async fn login(
    State(auth_svc): State<Arc<AuthService>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let req = LoginRequest {
        ip_address: Some(addr.ip()),
        user_agent: headers
            .get(axum::http::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string),
        ..body
    };
    let resp = auth_svc.login(req).await?;
    Ok((StatusCode::OK, Json(resp)))
}

// ── Refresh ───────────────────────────────────────────────────────────────────

pub async fn refresh(
    State(auth_svc): State<Arc<AuthService>>,
    Json(body): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    let resp = auth_svc.refresh(body).await?;
    Ok((StatusCode::OK, Json(resp)))
}

// ── Logout ────────────────────────────────────────────────────────────────────

pub async fn logout(
    State(auth_svc): State<Arc<AuthService>>,
    Json(body): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    auth_svc.logout(body).await?;
    Ok(StatusCode::NO_CONTENT)
}