use std::io;

use crate::domain::config::Config;
use thiserror::Error;

pub trait ConfigStore {
    fn load(&self) -> Result<Config, ConfigStoreError>;
    fn save(&self, config: &Config) -> Result<(), ConfigStoreError>;
}

#[derive(Debug, Error)]
pub enum ConfigStoreError {
    #[error("failed to save config")]
    FailedToSave,
    #[error("load failed: file does not exist")]
    FileDoesNotExist,
    #[error("malformed config")]
    MalformedConfig(#[from] serde_json::error::Error),
}

impl From<io::Error> for ConfigStoreError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => ConfigStoreError::FileDoesNotExist,
            _ => ConfigStoreError::FailedToSave,
        }
    }
}
