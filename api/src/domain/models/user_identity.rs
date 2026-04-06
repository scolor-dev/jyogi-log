use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserIdentity {
    pub id: i64,
    pub user_id: i64,
    pub identity_type: String,
    pub identifier: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserIdentity {
    pub fn is_username(&self) -> bool {
        self.identity_type == "username"
    }
}