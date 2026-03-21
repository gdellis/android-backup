use crate::error::{AppError, Result};
use log::{debug, info};
use std::path::PathBuf;
use std::process::Command;

pub struct AdbCommand {
    path: PathBuf,
    serial: Option<String>,
}

impl AdbCommand {
    #[allow(dead_code)]
    pub fn with_path(path: PathBuf) -> Self {
        Self { path, serial: None }
    }

    pub fn new() -> Result<Self> {
        let path = find_adb_path().ok_or(AppError::AdbNotFound)?;
        Ok(Self { path, serial: None })
    }

    pub fn with_serial(mut self, serial: String) -> Self {
        self.serial = Some(serial);
        self
    }

    pub fn get_devices(&self) -> Result<Vec<AdbDevice>> {
        let output = self
            .cmd()
            .arg("devices")
            .arg("-l")
            .output()
            .map_err(|e| AppError::Adb(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for line in stdout.lines().skip(1) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(device) = parse_device_line(line) {
                devices.push(device);
            }
        }

        Ok(devices)
    }

    pub fn backup(&self, output_path: &str, options: &BackupOptions) -> Result<()> {
        let mut cmd = self.cmd();
        cmd.arg("backup").arg("-f").arg(output_path);

        if options.all {
            cmd.arg("-all");
        }

        if options.apk {
            cmd.arg("-apk");
        }

        if options.compress {
            cmd.arg("-c");
        }

        if let Some(ref password) = options.password {
            cmd.arg(format!("-encrypt={}", password));
        } else {
            let password = generate_password();
            cmd.arg(format!("-encrypt={}", password));
            info!("Generated encryption password (backup will be encrypted)");
        }

        debug!("Executing: {:?}", cmd);

        let output = cmd.output().map_err(|e| AppError::Adb(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("FAILURE") {
                return Err(AppError::Backup(format!("Backup failed: {}", stderr)));
            }
        }

        Ok(())
    }

    pub fn restore(&self, backup_path: &str, password: Option<&str>) -> Result<()> {
        let mut cmd = self.cmd();
        cmd.arg("restore").arg(backup_path);

        if let Some(ref pwd) = password {
            cmd.arg(format!("-encrypt={}", pwd));
        }

        debug!("Executing: {:?}", cmd);

        let output = cmd.output().map_err(|e| AppError::Adb(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Restore(format!("Restore failed: {}", stderr)));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn version(&self) -> Result<String> {
        let output = self
            .cmd()
            .arg("version")
            .output()
            .map_err(|e| AppError::Adb(e.to_string()))?;

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(version)
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        if let Some(ref serial) = self.serial {
            cmd.arg("-s").arg(serial);
        }
        cmd
    }
}

fn find_adb_path() -> Option<PathBuf> {
    which_adb().or_else(fallback_adb_paths)
}

fn which_adb() -> Option<PathBuf> {
    std::process::Command::new("which")
        .arg("adb")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string().into())
            } else {
                None
            }
        })
}

fn fallback_adb_paths() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if cfg!(windows) {
        candidates.push(PathBuf::from(
            "C:\\Program Files\\Android\\android-sdk\\platform-tools\\adb.exe",
        ));
        candidates.push(PathBuf::from("C:\\Android\\platform-tools\\adb.exe"));
    } else {
        candidates.push(PathBuf::from("/usr/local/bin/adb"));
        candidates.push(PathBuf::from("/usr/bin/adb"));
        candidates.push(PathBuf::from("/opt/android-sdk/platform-tools/adb"));

        if let Some(home) = dirs::home_dir() {
            candidates.push(home.join("Android/Sdk/platform-tools/adb"));
            candidates.push(home.join(".android-sdk/platform-tools/adb"));
        }
    }

    candidates.into_iter().find(|p| p.exists())
}

fn parse_device_line(line: &str) -> Option<AdbDevice> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let serial = parts[0].to_string();
    let state = match parts[1] {
        "device" => DeviceState::Device,
        "unauthorized" => DeviceState::Unauthorized,
        "offline" => DeviceState::Offline,
        _ => return None,
    };

    let mut model = None;
    let mut product = None;
    let mut transport_id = None;

    for part in parts.iter().skip(2) {
        if let Some((key, value)) = part.split_once(':') {
            match key {
                "model" => model = Some(value.to_string()),
                "product" => product = Some(value.to_string()),
                "transport_id" => transport_id = Some(value.to_string()),
                _ => {}
            }
        }
    }

    Some(AdbDevice {
        serial,
        state,
        model,
        product,
        transport_id,
    })
}

fn generate_password() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    hex::encode(bytes)
}

#[derive(Debug, Clone)]
pub struct AdbDevice {
    pub serial: String,
    pub state: DeviceState,
    pub model: Option<String>,
    pub product: Option<String>,
    pub transport_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    Device,
    Unauthorized,
    Offline,
}

#[derive(Debug, Clone)]
pub struct BackupOptions {
    pub all: bool,
    pub apk: bool,
    pub compress: bool,
    pub password: Option<String>,
}

impl Default for BackupOptions {
    fn default() -> Self {
        Self {
            all: true,
            apk: false,
            compress: true,
            password: None,
        }
    }
}

impl Default for AdbCommand {
    fn default() -> Self {
        Self::new().expect("ADB not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_device_line_valid_device() {
        let line = "192.168.1.100:5555    device product:GT-N8010 model:GT_N8010 device:tuna";
        let device = parse_device_line(line);

        assert!(device.is_some());
        let device = device.unwrap();
        assert_eq!(device.serial, "192.168.1.100:5555");
        assert_eq!(device.state, DeviceState::Device);
        assert_eq!(device.model, Some("GT_N8010".to_string()));
        assert_eq!(device.product, Some("GT-N8010".to_string()));
    }

    #[test]
    fn test_parse_device_line_unauthorized() {
        let line = "192.168.1.100:5555    unauthorized";
        let device = parse_device_line(line);

        assert!(device.is_some());
        let device = device.unwrap();
        assert_eq!(device.serial, "192.168.1.100:5555");
        assert_eq!(device.state, DeviceState::Unauthorized);
        assert_eq!(device.model, None);
    }

    #[test]
    fn test_parse_device_line_offline() {
        let line = "192.168.1.100:5555    offline";
        let device = parse_device_line(line);

        assert!(device.is_some());
        let device = device.unwrap();
        assert_eq!(device.serial, "192.168.1.100:5555");
        assert_eq!(device.state, DeviceState::Offline);
    }

    #[test]
    fn test_parse_device_line_invalid() {
        assert!(parse_device_line("").is_none());
        assert!(parse_device_line("onlyone").is_none());
        assert!(parse_device_line("serial unknown_state").is_none());
        let result = parse_device_line("serial device model:foo");
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_device_line_with_transport_id() {
        let line = "12345678        device product:sdk_phone_x86_64 model:sdk_phone_x86_64 device:sdk_phone_x86_64 transport_id:1";
        let device = parse_device_line(line);

        assert!(device.is_some());
        let device = device.unwrap();
        assert_eq!(device.serial, "12345678");
        assert_eq!(device.state, DeviceState::Device);
        assert_eq!(device.transport_id, Some("1".to_string()));
    }

    #[test]
    fn test_backup_options_default() {
        let options = BackupOptions::default();
        assert!(options.all);
        assert!(!options.apk);
        assert!(options.compress);
        assert_eq!(options.password, None);
    }
}
