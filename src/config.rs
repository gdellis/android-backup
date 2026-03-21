use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub adb_path: Option<String>,
    pub backup_directory: Option<String>,
    pub default_apk: bool,
    pub default_compress: bool,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            adb_path: None,
            backup_directory: None,
            default_apk: false,
            default_compress: true,
            verbose: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)
                .map_err(|e| AppError::Config(format!("Failed to parse config: {}", e)))?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_path, contents)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_backup_directory(&self) -> PathBuf {
        self.backup_directory
            .clone()
            .map(PathBuf::from)
            .unwrap_or_else(get_default_backup_dir)
    }

    #[allow(dead_code)]
    pub fn get_adb_path(&self) -> Option<String> {
        self.adb_path.clone()
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::Config("Could not find config directory".to_string()))?;

    Ok(config_dir.join("android-backup").join("config.toml"))
}

#[allow(dead_code)]
pub fn get_default_backup_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("android-backup")
        .join("backups")
}
