use sqlx::PgPool;

use crate::adapter::security::token::jwt::JwtConfig;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt: JwtConfig,
}

impl AppState {
    pub fn new(pool: PgPool, jwt: JwtConfig) -> Self {
        Self { pool, jwt }
    }
}