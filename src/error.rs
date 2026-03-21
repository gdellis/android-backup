use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("ADB error: {0}")]
    Adb(String),

    #[error("Device error: {0}")]
    Device(String),

    #[error("Backup error: {0}")]
    Backup(String),

    #[error("Restore error: {0}")]
    Restore(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("No device found")]
    NoDeviceFound,

    #[error("Device not authorized. Please check the device and accept the authorization prompt.")]
    DeviceNotAuthorized,

    #[error("ADB not found. Please install ADB or provide the path with --adb-path")]
    AdbNotFound,

    #[error("Backup file not found: {0}")]
    BackupFileNotFound(String),

    #[error("Invalid backup format: {0}")]
    InvalidBackupFormat(String),

    #[error("User cancelled operation")]
    Cancelled,

    #[error("Timeout waiting for device")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, AppError>;
