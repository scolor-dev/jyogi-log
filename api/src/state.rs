use crate::config::Config;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    config: Config,
    db: PgPool,
}

impl AppState {
    #[must_use]
    pub fn new(config: Config, db: PgPool) -> Self {
        Self { config, db }
    }

    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    #[must_use]
    pub fn db(&self) -> &PgPool {
        &self.db
    }
}
