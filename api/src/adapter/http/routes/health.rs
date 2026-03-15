use axum::{routing::get, Router};

use crate::{
    adapter::http::handlers::health,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/health", get(health::health))
}