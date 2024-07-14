use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub public_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}