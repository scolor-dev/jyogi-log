pub mod admin;
pub mod auth;
pub mod clients;
pub mod health;
pub mod oauth;
pub mod users;

use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new().nest(
        "/api/v1",
        Router::<AppState>::new()
            .merge(health::routes())
            .merge(auth::routes())
            .merge(oauth::routes())
            .merge(clients::routes())
            .merge(users::routes())
            .merge(admin::routes()),
    )
}
