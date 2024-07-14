use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub data: Vec<u8>,
    pub entry_date: chrono::DateTime<chrono::Utc>,
}