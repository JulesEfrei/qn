use core::fmt;
use std::{
    fs::{self},
    path::PathBuf,
    process::Command,
};

use crate::{config::AppConfig, logger::log};

#[derive(Debug)]
pub enum FileSystemError {
    CreationError(String),
    DeletionError(String),
    NotFound,
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemError::NotFound => write!(f, "File not found."),
            FileSystemError::DeletionError(message) => {
                write!(f, "Unable to delete the file: {message}")
            }
            FileSystemError::CreationError(message) => {
                write!(f, "Error while creating file: {message}")
            }
        }
    }
}

impl std::error::Error for FileSystemError {}

pub struct Note {}

impl Note {
    pub fn create(config: &AppConfig, name: &String) -> Result<Note, FileSystemError> {
        let note_path = &config.notes_path;
        let file_name = format!("{name}.md");
        let file_path = note_path.join(&file_name);

        if let Ok(true) = fs::exists(&file_path) {
            log!("DEBUG", format!("File path: {:?}", file_path));
            Err(FileSystemError::CreationError(String::from(
                "File already exist",
            )))
        } else {
            fs::File::create(&file_path)
                .map_err(|e| FileSystemError::CreationError(e.to_string()))?;

            Self::open(&file_path, &config.editor);
            Ok(Note {})
        }
    }

    pub fn list() -> Result<Note, FileSystemError> {
        Ok(Note {})
    }

    pub fn delete(String: String) -> Result<(), FileSystemError> {
        Ok(())
    }

    fn open(file: &PathBuf, editor: &String) -> Option<()> {
        Command::new(editor)
            .arg(file.to_str().unwrap())
            .status()
            .expect("Failed to open editor");
        Some(())
    }
}
