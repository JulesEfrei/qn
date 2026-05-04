mod config;

use clap::{Parser, Subcommand};
use config::AppConfig;

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

macro_rules! log {
    ($level:expr, $message:expr) => {
        println!("[{}] {}", $level, $message);
    };

    ($level:expr, $message:expr, $file:expr, $line:expr) => {
        println!("[{}] {} (at {}:{})", $level, $message, $file, $line);
    };
}

fn main() {
    let config = AppConfig::load();

    println!("Notes path: {}", config.notes_path.display());
    println!("Editor: {}", config.editor);

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
