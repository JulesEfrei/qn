use core::fmt;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    DirectoryNotFound,
    WriteError(String),
    CreationError(String),
    ParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found."),
            ConfigError::DirectoryNotFound => write!(f, "Directory not found."),
            ConfigError::WriteError(message) => {
                write!(f, "Error while writting into the file: {message}")
            }
            ConfigError::CreationError(message) => {
                write!(f, "Error while creating file or directories: {message}")
            }
            ConfigError::ParseError(message) => {
                write!(f, "Error while parsing the configuration: {message}")
            }
        }
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
            let config =
                fs::read_to_string(&config_file).map_err(|_e| ConfigError::FileNotFound)?;

            let parsed = serde_yml::from_str::<AppConfig>(&config)
                .map_err(|e| ConfigError::ParseError(e.to_string()))?;

            Ok(parsed)
        } else {
            Self::setup_default_config(&config_file)
        }
    }

    fn get_project_directory() -> Result<PathBuf, ConfigError> {
        let project_directory = ProjectDirs::from("com", "bayne", "qn");

        match project_directory {
            None => Err(ConfigError::DirectoryNotFound),
            Some(project_directory) => Ok(project_directory.config_dir().to_path_buf()),
        }
    }

    fn get_config_file() -> Result<PathBuf, ConfigError> {
        Ok(Self::get_project_directory()?.join("config.yaml"))
    }

    fn get_document_directory() -> Result<PathBuf, ConfigError> {
        let user_dir = UserDirs::new();

        match user_dir {
            Some(user_dir) => {
                let document_dir = user_dir
                    .document_dir()
                    .ok_or(ConfigError::DirectoryNotFound)?;

                Ok(document_dir.to_path_buf())
            }
            None => {
                let base_dir = BaseDirs::new().ok_or(ConfigError::DirectoryNotFound)?;
                Ok(base_dir.home_dir().to_path_buf())
            }
        }
    }

    fn setup_default_config(config_file_path: &Path) -> Result<Self, ConfigError> {
        let note_directory = Self::get_document_directory()?.join("notes");

        let config_directory = config_file_path
            .parent()
            .ok_or(ConfigError::DirectoryNotFound)?;

        fs::create_dir_all(config_directory)
            .map_err(|e| ConfigError::CreationError(e.to_string()))?;

        fs::create_dir_all(&note_directory)
            .map_err(|e| ConfigError::CreationError(e.to_string()))?;

        let default_config = AppConfig {
            notes_path: note_directory,
            editor: std::env::var("EDITOR").unwrap_or_else(|_| String::from("nano")),
        };

        let yaml = serde_yml::to_string(&default_config)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        fs::write(config_file_path, yaml).map_err(|e| ConfigError::WriteError(e.to_string()))?;

        Ok(default_config)
    }
}
