use axum::{Router, routing::get};

use crate::{
    adapter::http::handlers::clients,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route(
            "/clients",
            get(clients::list_clients).post(clients::create_client),
        )
        .route(
            "/clients/{client_id}",
            get(clients::get_client)
                .patch(clients::update_client)
                .delete(clients::delete_client),
        )
}
