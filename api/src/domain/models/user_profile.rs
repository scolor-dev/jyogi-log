use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserProfile {
    pub id: i64,
    pub user_id: i64,
    pub user_uuid: Uuid,
    pub display_name: String,
    pub tagline: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}