use axum::Router;

use crate::{
    adapter::http::routes,
    state::AppState,
};

pub fn create_app(state: AppState) -> Router {
    Router::<AppState>::new()
        .merge(routes::routes())
        .with_state(state)
}
