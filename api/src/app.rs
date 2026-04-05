use axum::Router;

use crate::adapter::http::routes;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .merge(routes::health_route::router())
        .nest("/api/auth", routes::auth_routes::router())
        .with_state(state)
}