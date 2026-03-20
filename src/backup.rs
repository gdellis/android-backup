use crate::adb::{AdbCommand, BackupOptions};
use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct BackupManager {
    adb: AdbCommand,
}

impl BackupManager {
    pub fn new() -> Result<Self> {
        let adb = AdbCommand::new()?;
        Ok(Self { adb })
    }

    pub fn with_serial(serial: String) -> Result<Self> {
        let adb = AdbCommand::new()?.with_serial(serial);
        Ok(Self { adb })
    }

    pub fn create_backup(&self, output: &str, options: &BackupOptions) -> Result<BackupMetadata> {
        info!("Starting backup to: {}", output);

        let output_path = PathBuf::from(output);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        self.adb.backup(output, options)?;

        let metadata = BackupMetadata {
            version: 1,
            created_at: Utc::now(),
            device_serial: None,
            android_version: None,
            backup_type: if options.apk {
                "full_with_apk".to_string()
            } else {
                "full".to_string()
            },
            encrypted: options.password.is_some(),
            compressed: options.compress,
            size_bytes: fs::metadata(output).map(|m| m.len()).unwrap_or(0),
            app_count: None,
        };

        let metadata_path = output.to_string().replace(".ab", "_metadata.json");
        let metadata_json =
            serde_json::to_string_pretty(&metadata).map_err(|e| AppError::Backup(e.to_string()))?;
        fs::write(&metadata_path, metadata_json)?;

        info!("Backup completed: {}", output);
        Ok(metadata)
    }

    #[allow(dead_code)]
    pub fn restore_backup(&self, backup_path: &str, password: Option<&str>) -> Result<()> {
        info!("Starting restore from: {}", backup_path);

        if !std::path::Path::new(backup_path).exists() {
            return Err(AppError::BackupFileNotFound(backup_path.to_string()));
        }

        self.adb.restore(backup_path, password)?;

        info!("Restore completed successfully");
        Ok(())
    }

    #[allow(dead_code)]
    pub fn verify_backup(&self, backup_path: &str) -> Result<BackupMetadata> {
        let path = PathBuf::from(backup_path);
        if !path.exists() {
            return Err(AppError::BackupFileNotFound(backup_path.to_string()));
        }

        let metadata_path = backup_path.to_string().replace(".ab", "_metadata.json");
        if let Ok(contents) = fs::read_to_string(&metadata_path) {
            let metadata: BackupMetadata =
                serde_json::from_str(&contents).map_err(|e| AppError::Parse(e.to_string()))?;
            Ok(metadata)
        } else {
            let size = fs::metadata(&path)?.len();
            Ok(BackupMetadata {
                version: 1,
                created_at: Utc::now(),
                device_serial: None,
                android_version: None,
                backup_type: "unknown".to_string(),
                encrypted: false,
                compressed: false,
                size_bytes: size,
                app_count: None,
            })
        }
    }

    pub fn list_backups(directory: &str) -> Result<Vec<BackupInfo>> {
        let path = PathBuf::from(directory);
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        let mut backups = Vec::new();
        let entries = fs::read_dir(&path)?;

        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".ab") {
                let backup_path = entry.path();
                let metadata_path = backup_path.with_extension("ab_metadata.json");

                let created_at = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.created().ok())
                    .map(DateTime::<Utc>::from)
                    .unwrap_or_else(Utc::now);

                let size = fs::metadata(&backup_path).map(|m| m.len()).unwrap_or(0);

                let metadata = metadata_path
                    .exists()
                    .then(|| {
                        fs::read_to_string(&metadata_path)
                            .ok()
                            .and_then(|c| serde_json::from_str::<BackupMetadata>(&c).ok())
                    })
                    .flatten();

                backups.push(BackupInfo {
                    path: backup_path.to_string_lossy().to_string(),
                    name: file_name,
                    created_at,
                    size_bytes: size,
                    metadata,
                });
            }
        }

        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(backups)
    }

    #[allow(dead_code)]
    pub fn estimate_backup_size(&self) -> Result<u64> {
        let devices = self.adb.get_devices()?;
        if devices.is_empty() {
            return Err(AppError::NoDeviceFound);
        }

        Ok(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub device_serial: Option<String>,
    pub android_version: Option<String>,
    pub backup_type: String,
    pub encrypted: bool,
    pub compressed: bool,
    pub size_bytes: u64,
    pub app_count: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct BackupInfo {
    #[allow(dead_code)]
    pub path: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub size_bytes: u64,
    pub metadata: Option<BackupMetadata>,
}

impl std::fmt::Display for BackupInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size_mb = self.size_bytes as f64 / (1024.0 * 1024.0);
        write!(
            f,
            "{} ({:.2} MB) - {}",
            self.name,
            size_mb,
            self.created_at.format("%Y-%m-%d %H:%M")
        )
    }
}
