use axum::http::StatusCode;

pub async fn me() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}

pub async fn logout() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}

pub async fn refresh() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}
