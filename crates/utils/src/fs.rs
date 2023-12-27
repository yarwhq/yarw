use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("Failed to create directory")]
    CreateDir(#[from] std::io::Error),
}

pub fn create_dirs_all_vec(dirs: Vec<&Path>) -> Result<(), FsError> {
    for dir in dirs {
        if !dir.exists() {
            std::fs::create_dir_all(dir).map_err(FsError::CreateDir)?;
        }
    }
    Ok(())
}
