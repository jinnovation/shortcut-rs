use config::{Config, ConfigError};

use clap::{Parser, Subcommand};

use serde::Deserialize;

#[derive(Deserialize)]
#[allow(unused)]
pub(crate) struct ShortcutConfig {
    api_url: String,
    // api_key: String,
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
}

#[derive(Subcommand)]
enum EpicsSubcommand {
    List,
}

fn main() {
    let settings = ShortcutConfig::new().unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Stories { command }) => match *command {
            StoriesSubcommand::List => {
                println!("stories list")
            }
        },
        Some(Commands::Epics { command }) => match *command {
            EpicsSubcommand::List => {
                println!("epics list")
            }
        },
        None => {}
    }

    println!("api_url: {:?}", settings.api_url);
}
