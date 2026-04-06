use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub uuid: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.status == "active" && self.deleted_at.is_none()
    }

    pub fn is_pending(&self) -> bool {
        self.status == "pending" && self.deleted_at.is_none()
    }

    pub fn is_inactive(&self) -> bool {
        self.status == "inactive"
    }

    pub fn is_suspended(&self) -> bool {
        self.status == "suspended"
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}