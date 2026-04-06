use axum::Router;
use axum::routing::{get, post};

use crate::adapter::http::handlers::auth_handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(auth_handlers::signup))
        .route("/login", post(auth_handlers::login))
        .route("/refresh", post(auth_handlers::refresh))
        .route("/logout", post(auth_handlers::logout))
        .route("/me", get(auth_handlers::me))
}