use crate::logger::log;
use core::fmt;
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl ConfigError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error while loading the configuration:\n{}",
            self.message
        )
    }
}

impl std::error::Error for ConfigError {}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub notes_path: PathBuf,
    pub editor: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_file = Self::get_config_file()?;

        if config_file.exists() {
            log!(
                "INFO",
                format!("The config file exists at {}", config_file.display())
            );

            let config = fs::read_to_string(&config_file).map_err(|e| {
                ConfigError::new(format!(
                    "Unable to read config file {}: {e}",
                    config_file.display()
                ))
            })?;

            let parsed = serde_yml::from_str::<AppConfig>(&config).map_err(|e| {
                ConfigError::new(format!(
                    "Unable to parse config file {}: {e}",
                    config_file.display()
                ))
            })?;

            Ok(parsed)
        } else {
            log!("INFO", "The config file doesn't exist. Creating...");
            Self::setup_default_config(&config_file)
        }
    }

    fn get_project_directory() -> Result<PathBuf, ConfigError> {
        let project_directory = ProjectDirs::from("com", "bayne", "qn");

        match project_directory {
            None => Err(ConfigError::new("Unable to get the project directory")),
            Some(project_directory) => Ok(project_directory.config_dir().to_path_buf()),
        }
    }

    fn get_config_file() -> Result<PathBuf, ConfigError> {
        Ok(Self::get_project_directory()?.join("config.yaml"))
    }

    fn get_document_directory() -> Result<PathBuf, ConfigError> {
        let user_dir =
            UserDirs::new().ok_or_else(|| ConfigError::new("Unable to get the user directory"))?;

        let document_dir = user_dir
            .document_dir()
            .ok_or_else(|| ConfigError::new("Unable to get document directory"))?;

        Ok(document_dir.to_path_buf())
    }

    fn setup_default_config(file: &Path) -> Result<Self, ConfigError> {
        let note_directory = Self::get_document_directory()?.join("notes");

        let parent = file.parent().ok_or_else(|| {
            ConfigError::new(format!("Config path has no parent: {}", file.display()))
        })?;

        fs::create_dir_all(parent)
            .map_err(|e| ConfigError::new(format!("Unable to create config directory: {e}")))?;

        let default_config = AppConfig {
            notes_path: note_directory,
            editor: std::env::var("EDITOR").unwrap_or_else(|_| String::from("nano")),
        };

        let yaml = serde_yml::to_string(&default_config)
            .map_err(|e| ConfigError::new(format!("Unable to serialize config: {e}")))?;

        fs::write(file, yaml).map_err(|e| {
            ConfigError::new(format!(
                "Unable to write config file {}: {e}",
                file.display()
            ))
        })?;

        log!("DEBUG", format!("Config file location: {}", file.display()));

        fs::create_dir_all(&default_config.notes_path)
            .map_err(|e| ConfigError::new(format!("Unable to create notes directory: {e}")))?;

        Ok(default_config)
    }
}
