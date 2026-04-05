use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::adapter::http::validator::{validate_display_name, validate_password, validate_username};
use crate::service::auth_service;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: &'static str,
}

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
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn login() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn refresh() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn logout() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn me() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}