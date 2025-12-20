use serde::{Deserialize, Serialize};

pub mod client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: i32,
    pub name: String,
    pub owner_ids: Vec<String>,
    pub story_type: String,
    pub epic_id: Option<i32>,
}
