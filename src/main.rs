use config::{Config, ConfigError};

use clap::{Parser, Subcommand};

use serde::Deserialize;

use crate::shortcut::client::Client;

mod shortcut;

#[derive(Deserialize)]
#[allow(unused)]
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

#[derive(Parser)]
#[command(about = "Manage stories, epics, and projects in Shortcut.", long_about=None, arg_required_else_help=true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Work with Shortcut stories.")]
    Stories {
        #[command(subcommand)]
        command: StoriesSubcommand,
    },
    #[command(about = "Work with Shortcut epics.")]
    Epics {
        #[command(subcommand)]
        command: EpicsSubcommand,
    },
}

#[derive(Subcommand)]
enum StoriesSubcommand {
    List,
    Get { id: String },
}

#[derive(Subcommand)]
enum EpicsSubcommand {
    List,
}

fn main() -> Result<(), reqwest::Error> {
    let settings = ShortcutConfig::new().unwrap();

    let cli = Cli::parse();

    let client = Client {
        api_url: settings.api_url,
        api_key: settings.api_key,
    };

    match &cli.command {
        Some(Commands::Stories { command }) => match command {
            StoriesSubcommand::List => {
                println!("stories list");
                Ok(())
            }
            StoriesSubcommand::Get { id } => {
                println!("{:?}", client.get_story_by_id(id)?);
                Ok(())
            }
        },
        Some(Commands::Epics { command }) => match *command {
            EpicsSubcommand::List => {
                println!("epics list");
                Ok(())
            }
        },
        None => Ok(()),
    }
}
