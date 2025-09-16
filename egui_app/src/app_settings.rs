use std::{
    fs::{create_dir_all, OpenOptions},
    io::{self, Write},
};

use dirs::config_dir;
use egui_probe::EguiProbe;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(EguiProbe, Default, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    #[egui_probe(name = "Daily Notes Roots")]
    pub daily_notes_roots: Vec<String>,
    #[egui_probe(name = "If end time is missing, assume duration of (e.g. 30m, 1h)")]
    pub assume_duration_on_missing_end_time: Option<String>,
}

#[derive(Error, Debug)]
pub enum SaveError {
    #[error("Config directory not found")]
    ConfigDirNotFound,
    #[error("Could not open or create config file: {0}")]
    FailedToOpenOrCreateFile(io::Error),
    #[error("Toml serialization error: {0}")]
    TomlSerializationError(toml::ser::Error),
    #[error("Failed to write to file: {0}")]
    FailedWriteToFile(io::Error),
}

impl AppSettings {
    pub fn save(&self) -> Result<(), SaveError> {
        let dir = config_dir().ok_or(SaveError::ConfigDirNotFound)?;
        let config_file = dir.join("task_analyzer/config.toml");

        create_dir_all(config_file.parent().unwrap()).map_err(SaveError::FailedToOpenOrCreateFile)?;

        let serialized = toml::to_string_pretty(self).map_err(SaveError::TomlSerializationError)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_file)
            .map_err(SaveError::FailedToOpenOrCreateFile)?;

        file.write_all(serialized.as_bytes()).map_err(SaveError::FailedWriteToFile)

    }
}
