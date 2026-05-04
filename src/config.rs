use crate::logger::log;
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub notes_path: PathBuf,
    pub editor: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let project_directory =
            ProjectDirs::from("com", "user", "qn").expect("Unable to get project directory");
        let config_directory = project_directory.config_dir();
        let config_file = config_directory.join("config.yaml");

        if config_file.exists() {
            log!(
                "INFO",
                format!("The config file exist at {}", config_file.display())
            );
            let config = fs::read_to_string(config_file).expect("Unable to read config file");
            serde_yml::from_str(&config).expect("Unable to parse config file")
        } else {
            log!("INFO", "The config file don't exist.\n Creating...");
            Self::setup_default_config(&config_file)
        }
    }

    fn setup_default_config(file: &Path) -> Self {
        let user_dir = UserDirs::new().expect("Unable to get the user directory");
        let document_dir = user_dir
            .document_dir()
            .expect("Unable to get document directory");
        let note_directory = document_dir.join("notes");

        fs::create_dir_all(&note_directory).ok();

        let default_config = AppConfig {
            notes_path: note_directory,
            editor: std::env::var("EDITOR").unwrap_or_else(|_| String::from("nano")),
        };

        let yaml = serde_yml::to_string(&default_config).unwrap();

        fs::write(file, yaml).ok();
        log!("DEBUG", format!("Config file location: {}", file.display()));
        fs::create_dir_all(&default_config.notes_path).ok();

        default_config
    }
}
