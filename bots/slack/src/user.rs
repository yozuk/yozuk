use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Identity {
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub tz: Option<String>,
}
