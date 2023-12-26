use directories::UserDirs;
use std::path::PathBuf;

lazy_static::lazy_static! {
    pub static ref CACHE: PathBuf = {
        let user_dirs = UserDirs::new();
        let cache_dir = user_dirs
            .map(|_dirs| directories::ProjectDirs::from("net", "yarwhq", "yarw")
                .map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
                .unwrap_or_default())
            .unwrap_or_default();

        cache_dir
    };
    pub static ref CONFIG: PathBuf = {
        let user_dirs = UserDirs::new();
        let config_dir = user_dirs
            .map(|_dirs| directories::ProjectDirs::from("net", "yarwhq", "yarw")
                .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
                .unwrap_or_default())
            .unwrap_or_default();

        config_dir
    };
}
