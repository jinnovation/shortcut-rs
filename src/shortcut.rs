use serde::{Deserialize, Serialize};

pub mod client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: i64,
    pub name: String,
    pub owner_ids: Vec<String>,
}
