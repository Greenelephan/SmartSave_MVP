use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub setting_key: String,
    pub setting_value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
