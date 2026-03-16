use axum::{Router, routing::get};

use crate::{
    adapter::http::handlers::users,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new().route("/users/me", get(users::me))
}
