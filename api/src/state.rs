use std::sync::Arc;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    db: Arc<PgPool>,
}

impl AppState {
    #[must_use]
    pub fn new(db: PgPool) -> Self {
        Self { db: Arc::new(db) }
    }

    #[must_use]
    pub fn db(&self) -> &PgPool {
        &self.db
    }
}
