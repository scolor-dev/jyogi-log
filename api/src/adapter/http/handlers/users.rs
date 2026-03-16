use axum::http::StatusCode;

pub async fn me() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}
