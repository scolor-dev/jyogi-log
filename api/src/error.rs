use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{error::Error as StdError, fmt, net::AddrParseError};

#[derive(Debug)]
pub enum ApiError {
    InternalServerError,
    Unauthorized,
    Conflict,
    BadRequest,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error"),
            Self::Unauthorized        => (StatusCode::UNAUTHORIZED, "unauthorized"),
            Self::Conflict            => (StatusCode::CONFLICT, "conflict"),
            Self::BadRequest          => (StatusCode::BAD_REQUEST, "bad_request"),
        };
        (status, axum::Json(serde_json::json!({ "error": error }))).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db_err) = e
            && db_err.code().as_deref() == Some("23505")
        {
            return Self::Conflict;
        }
        tracing::error!("database error: {:?}", e);
        Self::InternalServerError
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(e: bcrypt::BcryptError) -> Self {
        tracing::error!("bcrypt error: {:?}", e);
        Self::InternalServerError
    }
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidPort {
        value: String,
        source: std::num::ParseIntError,
    },
    InvalidListenAddress {
        host: String,
        port: u16,
        source: AddrParseError,
    },
    MissingEnvVar {
        name: &'static str,
    },
    InvalidDbMaxConnections {
        value: String,
        source: std::num::ParseIntError,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort { value, .. } => {
                write!(f, "APP_PORT must be a valid u16, got `{value}`")
            }
            Self::InvalidListenAddress { host, port, .. } => {
                write!(f, "invalid listen address `{host}:{port}`")
            }
            Self::MissingEnvVar { name } => {
                write!(f, "required environment variable `{name}` is not set")
            }
            Self::InvalidDbMaxConnections { value, .. } => {
                write!(f, "DB_MAX_CONNECTIONS must be a valid u32, got `{value}`")
            }
        }
    }
}

impl StdError for ConfigError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidPort { source, .. }
            | Self::InvalidDbMaxConnections { source, .. } => Some(source),
            Self::InvalidListenAddress { source, .. } => Some(source),
            Self::MissingEnvVar { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Io(std::io::Error),
    Database(sqlx::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(error) => write!(f, "{error}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Database(error) => write!(f, "{error}"),
        }
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Config(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Database(error) => Some(error),
        }
    }
}

impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}
