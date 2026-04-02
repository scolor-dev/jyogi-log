use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
    adapter::http::handlers::auth_handler::{login, logout, refresh, signup},
    service::auth_service::AuthService,
};

/// `/api/auth` 以下のルーティング。
/// app.rs で `.nest("/api/auth", auth_routes(auth_svc))` として登録する。
pub fn auth_routes(auth_svc: Arc<AuthService>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .with_state(auth_svc)
}