use config::{Config, ConfigError};

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ShortcutConfig {
    api_url: String,
    api_key: String,
}

impl ShortcutConfig {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(config::Environment::with_prefix("SHORTCUT"))
            .set_default("api_url", "https://api.app.shortcut.com/api/v3")?
            .build()?
            .try_deserialize()
    }
}

fn main() {
    let settings = ShortcutConfig::new().unwrap();

    println!("api_url: {:?}", settings.api_url);
}
