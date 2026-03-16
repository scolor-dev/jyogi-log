use axum::{Router, routing::get};

use crate::{
    adapter::http::handlers::admin,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/admin/users", get(admin::list_users))
        .route("/admin/clients", get(admin::list_clients))
        .route("/admin/audit-logs", get(admin::list_audit_logs))
}
