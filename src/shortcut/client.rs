use reqwest::{Error, header::CONTENT_TYPE};

use crate::shortcut::Story;

#[derive(Debug)]
pub struct Client {
    pub api_url: String,
    pub api_key: String,
}

impl Client {
    pub fn get_story_by_id(&self, id: &str) -> Result<Story, Error> {
        let client = reqwest::blocking::Client::new();

        let story: Story = client
            .get(format!("{0}/stories/{id}", self.api_url))
            .header("Shortcut-Token", &self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .send()?
            .error_for_status()?
            .json()?;

        Ok(story)
    }
}
