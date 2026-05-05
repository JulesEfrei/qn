mod config;
mod logger;

use clap::{Parser, Subcommand};
use config::AppConfig;
use logger::log;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new quick note
    New {
        /// Name of the new note
        name: String,
    },
}

fn main() {
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            log!("ERROR", format!("{}", e));
            std::process::exit(1);
        }
    };

    log!(
        "DEBUG",
        format!("Notes path: {}", config.notes_path.display())
    );
    log!("DEBUG", format!("Editor: {}", config.editor));

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New { name }) => {
            println!("'myapp add' was used, name is: {name:?}");
        }
        None => {
            log!("INFO", "Running TUI application");
        }
    }
}
