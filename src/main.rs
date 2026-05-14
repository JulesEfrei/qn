mod config;
mod logger;
mod notes;

use clap::{Parser, Subcommand};
use config::AppConfig;
use logger::log;
use notes::Note;

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

    /// List all notes
    List,

    /// Delete specific note
    Delete {
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
            Note::create(&config, name).expect("arh");
        }
        Some(Commands::List) => {
            println!("List notes");
        }
        Some(Commands::Delete { name }) => {
            println!("Delete following note: {name:?}");
        }
        None => {
            log!("INFO", "Running TUI application");
        }
    }
}
