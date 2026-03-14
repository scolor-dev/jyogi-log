use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum ApiError {
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}