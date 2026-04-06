use crate::adapter::security::token::jwt::JwtConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt: JwtConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/app".to_string()),
            jwt: JwtConfig {
                secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "changeme_secret_key".to_string()),
                expires_in_secs: std::env::var("JWT_EXPIRES_IN_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600),
            },
        }
    }
}