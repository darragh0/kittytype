use crate::settings::error::SettingsError;
use dirs::config_local_dir;
use std::fs;
use std::path::PathBuf;

pub fn config_path() -> PathBuf {
    config_local_dir()
        .unwrap_or_else(|| PathBuf::from(".")) // fallback to cur dir
        .join("kittytype")
        .join("config.json")
}

pub fn read_config_json() -> Result<String, SettingsError> {
    fs::read_to_string(config_path()).map_err(SettingsError::IOError)
}

pub fn write_config_json(json: &str) -> Result<(), SettingsError> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(SettingsError::IOError)?;
    }
    fs::write(path, json).map_err(SettingsError::IOError)
}
