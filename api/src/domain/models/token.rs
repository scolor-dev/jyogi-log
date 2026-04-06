use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub session_uuid: Uuid,
}