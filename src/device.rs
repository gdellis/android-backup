use crate::adb::{AdbCommand, AdbDevice, DeviceState};
use crate::error::{AppError, Result};
use log::info;

pub struct DeviceManager {
    adb: AdbCommand,
}

impl DeviceManager {
    pub fn new() -> Result<Self> {
        let adb = AdbCommand::new()?;
        Ok(Self { adb })
    }

    #[allow(dead_code)]
    pub fn with_adb_path(path: std::path::PathBuf) -> Result<Self> {
        let adb = AdbCommand::with_path(path);
        Ok(Self { adb })
    }

    pub fn list_devices(&self) -> Result<Vec<AdbDevice>> {
        let devices = self.adb.get_devices()?;
        Ok(devices)
    }

    #[allow(dead_code)]
    pub fn find_device(&self, serial: Option<&str>) -> Result<AdbDevice> {
        let devices = self.list_devices()?;

        if devices.is_empty() {
            return Err(AppError::NoDeviceFound);
        }

        match serial {
            Some(s) => devices
                .into_iter()
                .find(|d| d.serial == s)
                .ok_or_else(|| AppError::Device(format!("Device with serial {} not found", s))),
            None => {
                let authorized: Vec<_> = devices
                    .iter()
                    .filter(|d| d.state == DeviceState::Device)
                    .cloned()
                    .collect();

                if authorized.is_empty() {
                    let has_unauthorized =
                        devices.iter().any(|d| d.state == DeviceState::Unauthorized);

                    if has_unauthorized {
                        return Err(AppError::DeviceNotAuthorized);
                    }
                    return Err(AppError::NoDeviceFound);
                }

                if authorized.len() == 1 {
                    Ok(authorized.into_iter().next().unwrap())
                } else {
                    Err(AppError::Device(
                        "Multiple devices found. Please specify a device serial.".to_string(),
                    ))
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn wait_for_device(&self, timeout_secs: u64) -> Result<AdbDevice> {
        use std::time::{Duration, Instant};

        let start = Instant::now();
        let interval = Duration::from_secs(1);

        while start.elapsed().as_secs() < timeout_secs {
            match self.find_device(None) {
                Ok(device) => {
                    info!("Device found: {:?}", device.serial);
                    return Ok(device);
                }
                Err(AppError::NoDeviceFound) | Err(AppError::DeviceNotAuthorized) => {
                    std::thread::sleep(interval);
                }
                Err(e) => return Err(e),
            }
        }

        Err(AppError::Timeout)
    }

    #[allow(dead_code)]
    pub fn get_device_info(&self, serial: &str) -> Result<DeviceInfo> {
        let adb = AdbCommand::new()?.with_serial(serial.to_string());
        let devices = adb
            .get_devices()?
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Device(format!("Device {} not found", serial)))?;

        Ok(DeviceInfo {
            serial: devices.serial,
            model: devices.model.unwrap_or_else(|| "Unknown".to_string()),
            product: devices.product.unwrap_or_else(|| "Unknown".to_string()),
            state: devices.state,
        })
    }

    #[allow(dead_code)]
    pub fn is_device_authorized(&self, serial: &str) -> Result<bool> {
        let devices = self.list_devices()?;
        Ok(devices
            .iter()
            .any(|d| d.serial == serial && d.state == DeviceState::Device))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub serial: String,
    pub model: String,
    pub product: String,
    pub state: DeviceState,
}

impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) [{}]",
            self.model,
            self.serial,
            match self.state {
                DeviceState::Device => "authorized",
                DeviceState::Unauthorized => "unauthorized",
                DeviceState::Offline => "offline",
            }
        )
    }
}
