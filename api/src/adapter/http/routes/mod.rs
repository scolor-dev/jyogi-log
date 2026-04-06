pub mod auth_routes;
pub mod health_route;

use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .merge(health_route::router())
        .nest("/auth", auth_routes::router())
}