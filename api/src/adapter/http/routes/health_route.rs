use axum::{routing::get, Router};

use crate::adapter::http::handlers::health_handler::health_check;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}