use axum::{routing::{get, post}, Router};

use crate::adapter::http::handlers::auth_handlers::{login, logout, me, refresh, signup};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(me))
}