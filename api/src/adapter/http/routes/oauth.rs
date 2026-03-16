use axum::{Router, routing::{get, post}};

use crate::{
    adapter::http::handlers::oauth,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/oauth/authorize", get(oauth::authorize))
        .route("/oauth/token", post(oauth::token))
        .route("/oauth/revoke", post(oauth::revoke))
        .route("/oauth/userinfo", get(oauth::userinfo))
}
