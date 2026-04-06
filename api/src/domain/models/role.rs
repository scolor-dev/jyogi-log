use chrono::{DateTime, Utc};

pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_USER: &str = "user";

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Permission {
    pub id: i64,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RolePermission {
    pub role_id: i64,
    pub permission_id: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRole {
    pub user_id: i64,
    pub role_id: i64,
    pub created_at: DateTime<Utc>,
}