use axum::{Router, routing::{get, post}};

use crate::{
    adapter::http::handlers::auth,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/auth/me", get(auth::me))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/refresh", post(auth::refresh))
}
