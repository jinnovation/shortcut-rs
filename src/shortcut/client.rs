use reqwest::{Error, header::CONTENT_TYPE};

use crate::{ShortcutConfig, shortcut::Story};

#[derive(Debug)]
pub struct Client {
    pub config: ShortcutConfig,

    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(config: ShortcutConfig) -> Self {
        Client {
            config,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_story_by_id(&self, id: &str) -> Result<Story, Error> {
        let story = self
            .client
            .get(format!("{0}/stories/{id}", self.config.api_url))
            .header("Shortcut-Token", &self.config.api_key)
            .header(CONTENT_TYPE, "application/json")
            .send()?
            .error_for_status()?
            .json()?;

        Ok(story)
    }
}
