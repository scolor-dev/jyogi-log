use axum::Router;

use crate::adapter::http::routes;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .merge(routes::health::router())
        .with_state(state)
}