use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub public_key: [u8; 32],
    pub created_at: chrono::DateTime<chrono::Utc>,
}