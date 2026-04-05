use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserCredential {
    pub id: i64,
    pub user_id: i64,
    pub credential_type: String,
    pub secret: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}