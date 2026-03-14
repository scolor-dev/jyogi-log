use axum::{routing::get, Router};

use crate::adapter::http::handlers::health;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
}