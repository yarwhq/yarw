use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

use crate::dirs::CONFIG;

use yarw_utils::fs::{create_dirs_all_vec, FsError};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error occurred with DB: {0}")]
    DbError(#[from] sled::Error),

    #[error("Error occurred with bincode: {0}")]
    BincodeError(#[from] bincode::Error),

    #[error("Error occurred with utils: {0}")]
    FsUtilsError(#[from] FsError),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Defaults {
    pub wineroot_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub defaults: Defaults,
}

lazy_static::lazy_static! {
    static ref DB_PATH: PathBuf = CONFIG.join("config");
}

impl Config {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self)
            .map_err(ConfigError::BincodeError)
            .unwrap()
    }

    fn deserialize(serialized_data: &[u8]) -> Self {
        bincode::deserialize(serialized_data)
            .map_err(ConfigError::BincodeError)
            .unwrap()
    }

    /// Update values in Config/Defaults struct
    pub fn update_defaults(&mut self, new_defaults: Defaults) {
        self.defaults = new_defaults;
    }

    pub fn load() -> Result<Self, ConfigError> {
        create_dirs_all_vec(vec![CONFIG.as_path()]).map_err(ConfigError::FsUtilsError)?;
        let tree = sled::open(DB_PATH.as_path()).map_err(ConfigError::DbError)?;

        if let Some(serialized_data) = tree.get(b"config").map_err(ConfigError::DbError)? {
            Ok(Config::deserialize(&serialized_data))
        } else {
            Ok(Config::default())
        }
    }

    /// Inserts values into the DB, then flushes changes to disk
    pub fn save(&self) -> Result<(), ConfigError> {
        let tree = sled::open(DB_PATH.as_path()).map_err(ConfigError::DbError)?;

        tree.insert(b"config", self.serialize())
            .map_err(ConfigError::DbError)?;
        tree.flush().map_err(ConfigError::DbError)?;
        Ok(())
    }
}
