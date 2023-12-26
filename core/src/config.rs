use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

use crate::dirs::CONFIG;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error occured with DB: {0}")]
    DbError(#[from] sled::Error),

    #[error("Error occured with bincode: {0}")]
    BincodeError(#[from] bincode::Error),

    #[error("Profile does not exist")]
    ProfileNotExist,
}

/// Examples:
///
/// ```rust
/// // Load Config
/// let mut config = Config::load()
///
/// // You can use values directly from the Config
/// println!("{:?}", config.defaults.wineroot_path);
///
/// // Change values in Config
/// // in this case, we're changing the values for "Defaults" struct
/// let new_defaults = Defaults {
///     wineroot_path: String::from("/wineroot"),
/// };
/// config.update_defaults(new_defaults);
///
/// config.save()
/// ```
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub defaults: Defaults,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Defaults {
    pub wineroot_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Profile {
    pub name: String,
    pub roblox: RobloxType,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum RobloxType {
    #[default]
    RobloxPlayer,
    RobloxStudio,
}

lazy_static::lazy_static! {
    pub static ref DB_PATH: PathBuf = CONFIG.join("db");
}

impl Config {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).map_err(ConfigError::BincodeError).unwrap()
    }

    fn deserialize(serialized_data: &[u8]) -> Self {
        bincode::deserialize(serialized_data).map_err(ConfigError::BincodeError).unwrap()
    }

    /// Update values in Config/Defaults struct
    pub fn update_defaults(&mut self, new_defaults: Defaults) {
        self.defaults = new_defaults;
    }

    /// Examples:
    /// 
    /// ```rust
    /// let new_profile = Profile {
    ///     name: String::from("name"),
    ///     roblox: RobloxType::RobloxPlayer,
    /// };
    /// config.create_profile(new_profile);
    /// ```
    pub fn create_profile(&mut self, profile: Profile) -> Result<Uuid, ConfigError> {
        let uuid = Uuid::new_v4();
    
        self.profiles.insert(uuid.to_string(), profile);
        Ok(uuid)
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn update_profile(&mut self, profile_uuid: Uuid, updated_profile: Profile) -> Result<(), ConfigError> {
        if let Some(existing_profile) = self.profiles.get_mut(&profile_uuid.to_string()) {
            *existing_profile = updated_profile;
            Ok(())
        } else {
            Err(ConfigError::ProfileNotExist)
        }
    }
    
    pub fn load() -> Result<Self, ConfigError> {
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

        tree.insert(b"config", self.serialize()).map_err(ConfigError::DbError)?;
        tree.flush().map_err(ConfigError::DbError)?;
        Ok(())
    }
}
