use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;
use uuid::Uuid;

use crate::dirs::CONFIG;

use yarw_utils::fs::{create_dirs_all_vec, FsError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Profile {
    pub name: String,
    pub roblox: RobloxType,
    pub fflags: HashMap<String, FFlagValue>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum RobloxType {
    #[default]
    RobloxPlayer,
    RobloxStudio,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FFlagValue {
    String(String),
    Number(i64),
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProfileManager {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("Error occurred with DB: {0}")]
    DbError(#[from] sled::Error),

    #[error("Error occurred with bincode: {0}")]
    BincodeError(#[from] bincode::Error),

    #[error("Error occurred with utils: {0}")]
    FsUtilsError(#[from] FsError),

    #[error("Profile does not exist")]
    ProfileNotExist,
}

lazy_static::lazy_static! {
    static ref DB_PATH: PathBuf = CONFIG.join("profiles");
}

impl ProfileManager {
    pub fn create(&mut self, profile: Profile) -> Result<Uuid, ProfileError> {
        let uuid = Uuid::new_v4();
        self.profiles.insert(uuid.to_string(), profile);
        Ok(uuid)
    }

    pub fn delete(&mut self, profile_uuid: Uuid) -> Result<(), ProfileError> {
        if self.profiles.remove(&profile_uuid.to_string()).is_some() {
            Ok(())
        } else {
            Err(ProfileError::ProfileNotExist)
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn update(
        &mut self,
        profile_uuid: Uuid,
        updated_profile: Profile,
    ) -> Result<(), ProfileError> {
        if let Some(existing_profile) = self.profiles.get_mut(&profile_uuid.to_string()) {
            *existing_profile = updated_profile;
            Ok(())
        } else {
            Err(ProfileError::ProfileNotExist)
        }
    }

    pub fn load() -> Result<Self, ProfileError> {
        create_dirs_all_vec(vec![CONFIG.as_path()]).map_err(ProfileError::FsUtilsError)?;
        let tree = sled::open(DB_PATH.as_path()).map_err(ProfileError::DbError)?;

        if let Some(serialized_data) = tree.get(b"profiles").map_err(ProfileError::DbError)? {
            Ok(ProfileManager::deserialize(&serialized_data))
        } else {
            Ok(ProfileManager::default())
        }
    }

    pub fn save(&self) -> Result<(), ProfileError> {
        let tree = sled::open(DB_PATH.as_path()).map_err(ProfileError::DbError)?;

        tree.insert(b"profiles", self.serialize())
            .map_err(ProfileError::DbError)?;
        tree.flush().map_err(ProfileError::DbError)?;
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self)
            .map_err(ProfileError::BincodeError)
            .unwrap()
    }

    fn deserialize(serialized_data: &[u8]) -> Self {
        bincode::deserialize(serialized_data)
            .map_err(ProfileError::BincodeError)
            .unwrap()
    }
}
