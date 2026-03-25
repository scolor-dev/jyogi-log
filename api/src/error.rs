use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{error::Error as StdError, fmt, net::AddrParseError};

#[derive(Debug)]
pub enum ApiError {
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
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
        }
    }
}

impl StdError for ConfigError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidPort { source, .. } => Some(source),
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
