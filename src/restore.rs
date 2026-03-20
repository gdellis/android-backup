use crate::adb::AdbCommand;
use crate::error::{AppError, Result};
use log::info;
use std::path::PathBuf;

pub struct RestoreManager {
    adb: AdbCommand,
}

impl RestoreManager {
    pub fn new() -> Result<Self> {
        let adb = AdbCommand::new()?;
        Ok(Self { adb })
    }

    pub fn with_serial(serial: String) -> Result<Self> {
        let adb = AdbCommand::new()?.with_serial(serial);
        Ok(Self { adb })
    }

    pub fn restore(&self, backup_path: &str, password: Option<&str>) -> Result<()> {
        info!("Starting restore from: {}", backup_path);

        let path = PathBuf::from(backup_path);
        if !path.exists() {
            return Err(AppError::BackupFileNotFound(backup_path.to_string()));
        }

        self.adb.restore(backup_path, password)?;

        info!("Restore completed successfully");
        Ok(())
    }

    pub fn validate_backup(&self, backup_path: &str) -> Result<BackupValidation> {
        let path = PathBuf::from(backup_path);
        if !path.exists() {
            return Err(AppError::BackupFileNotFound(backup_path.to_string()));
        }

        let file_size = std::fs::metadata(&path)?.len();

        if file_size == 0 {
            return Ok(BackupValidation {
                valid: false,
                error: Some("Backup file is empty".to_string()),
                size_bytes: 0,
            });
        }

        if file_size < 100 {
            return Ok(BackupValidation {
                valid: false,
                error: Some("Backup file is too small to be valid".to_string()),
                size_bytes: file_size,
            });
        }

        let file_header: Option<[u8; 4]> = std::fs::File::open(&path).ok().and_then(|mut f| {
            let mut header = [0u8; 4];
            std::io::Read::read_exact(&mut f, &mut header).ok()?;
            Some(header)
        });

        let is_android_backup = file_header
            .map(|h| {
                let header = [h[0], h[1], h[2], h[3]];
                header == [0x41, 0x4e, 0x44, 0x52] || h[..2] == [0x1f, 0x8b]
            })
            .unwrap_or(false);

        if !is_android_backup {
            return Ok(BackupValidation {
                valid: false,
                error: Some("File does not appear to be a valid Android backup".to_string()),
                size_bytes: file_size,
            });
        }

        Ok(BackupValidation {
            valid: true,
            error: None,
            size_bytes: file_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BackupValidation {
    pub valid: bool,
    pub error: Option<String>,
    pub size_bytes: u64,
}
