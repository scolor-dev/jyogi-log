use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RefreshToken {
    pub id: i64,
    pub token_hash: String,
    pub session_id: i64,
    pub user_id: i64,
    pub user_uuid: Uuid,
    pub session_uuid: Uuid,
    pub is_used: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_used && !self.is_expired()
    }
}