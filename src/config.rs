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
        let config_file = Self::get_config_file();

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

    fn get_project_directory() -> PathBuf {
        let project_directory =
            ProjectDirs::from("com", "bayne", "qn").expect("Unable to get project directory");
        project_directory.config_dir().to_path_buf()
    }

    fn get_config_file() -> PathBuf {
        Self::get_project_directory().join("config.yaml")
    }

    fn get_document_directory() -> PathBuf {
        let user_dir = UserDirs::new().expect("Unable to get the user directory");
        let document_dir = user_dir
            .document_dir()
            .expect("Unable to get document directory");
        document_dir.to_path_buf()
    }

    fn setup_default_config(file: &Path) -> Self {
        let note_directory = Self::get_document_directory().join("notes");

        fs::create_dir_all(file.parent().unwrap()).ok();

        let default_config = AppConfig {
            notes_path: note_directory,
            editor: std::env::var("EDITOR").unwrap_or_else(|_| String::from("nano")),
        };

        let yaml = serde_yml::to_string(&default_config).unwrap();

        fs::write(file, yaml).expect("Unable to write config file");
        log!("DEBUG", format!("Config file location: {}", file.display()));
        fs::create_dir_all(&default_config.notes_path).ok();

        default_config
    }
}
