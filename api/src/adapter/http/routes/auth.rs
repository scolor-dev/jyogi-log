use axum::{Router, routing::{get, post}};

use crate::{
    adapter::http::handlers::auth,
    state::AppState,
};

/// 認証関連ルートを返す
pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/auth/signup", post(auth::signup))
        .route("/auth/login",  post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me",     get(auth::me))
}
