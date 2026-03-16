use axum::http::StatusCode;

pub async fn authorize() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}

pub async fn token() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}

pub async fn revoke() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}

pub async fn userinfo() -> (StatusCode, &'static str) {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}
