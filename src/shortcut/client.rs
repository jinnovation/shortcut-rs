use reqwest::Error;

use crate::shortcut::Story;

#[derive(Debug)]
pub struct Client {
    pub api_url: String,
}

impl Client {
    pub fn get_story_by_id(&self, _id: &str) -> Result<Story, Error> {
        println!("api_url: {:?}", self.api_url);
        Ok(Story {
            id: 3,
            name: "foobar".to_string(),
            owner_ids: vec!["123".to_string()],
        })
    }
}
