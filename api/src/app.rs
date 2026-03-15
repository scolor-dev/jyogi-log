use axum::Router;

use crate::{
    adapter::http::routes::health,
    state::AppState,
};

pub fn create_app(state: AppState) -> Router {
    Router::<AppState>::new()
        .merge(health::routes())
        .with_state(state)
}