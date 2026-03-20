mod adb;
mod backup;
mod cli;
mod crypto;
mod device;
mod error;
mod restore;

use anyhow::Result;
use log::{error, info, LevelFilter};

use crate::adb::BackupOptions;
use crate::backup::BackupManager;
use crate::cli::Cli;
use crate::device::DeviceManager;
use crate::restore::RestoreManager;

fn init_logging(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp_secs()
        .init();
}

fn get_default_backup_dir() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("android-backup")
        .join("backups")
}

fn get_default_backup_filename() -> String {
    let now = chrono::Local::now();
    format!("backup_{}.ab", now.format("%Y%m%d_%H%M%S"))
}

fn handle_backup(
    cli: &Cli,
    output: &Option<String>,
    apk: &bool,
    _compress: &bool,
    _all: &bool,
) -> Result<()> {
    let backup_manager = if let Some(ref serial) = cli.serial {
        BackupManager::with_serial(serial.clone())?
    } else {
        BackupManager::new()?
    };

    let output = output.clone().unwrap_or_else(|| {
        get_default_backup_dir()
            .join(get_default_backup_filename())
            .to_string_lossy()
            .to_string()
    });

    let options = BackupOptions {
        all: true,
        apk: *apk,
        compress: true,
        password: None,
    };

    info!("Starting backup to: {}", output);
    let metadata = backup_manager.create_backup(&output, &options)?;
    info!("Backup completed: {} bytes", metadata.size_bytes);
    println!("Backup saved to: {}", output);
    println!(
        "Size: {:.2} MB",
        metadata.size_bytes as f64 / (1024.0 * 1024.0)
    );
    println!("Encrypted: {}", metadata.encrypted);

    Ok(())
}

fn handle_restore(cli: &Cli, backup_file: &String, password: &Option<String>) -> Result<()> {
    let restore_manager = if let Some(ref serial) = cli.serial {
        RestoreManager::with_serial(serial.clone())?
    } else {
        RestoreManager::new()?
    };

    info!("Starting restore from: {}", backup_file);
    restore_manager.restore(backup_file, password.as_deref())?;
    println!("Restore completed successfully");

    Ok(())
}

fn handle_devices(detailed: &bool) -> Result<()> {
    let device_manager = DeviceManager::new()?;
    let devices = device_manager.list_devices()?;

    if devices.is_empty() {
        println!("No devices found");
        return Ok(());
    }

    println!("Connected devices:");
    for device in &devices {
        if *detailed {
            println!(
                "  {} - {} (state: {:?})",
                device.serial,
                device.model.as_deref().unwrap_or("Unknown"),
                device.state
            );
            if let Some(ref product) = device.product {
                println!("    Product: {}", product);
            }
            if let Some(ref transport_id) = device.transport_id {
                println!("    Transport ID: {}", transport_id);
            }
        } else {
            println!(
                "  {} ({}) [{}]",
                device.serial,
                device.model.as_deref().unwrap_or("Unknown"),
                match device.state {
                    adb::DeviceState::Device => "device",
                    adb::DeviceState::Unauthorized => "unauthorized",
                    adb::DeviceState::Offline => "offline",
                }
            );
        }
    }

    Ok(())
}

fn handle_list(directory: &String, detailed: &bool) -> Result<()> {
    let backups = BackupManager::list_backups(directory)?;

    if backups.is_empty() {
        println!("No backups found in {}", directory);
        return Ok(());
    }

    println!("Backups in {}:", directory);
    for backup in &backups {
        if *detailed {
            println!("  {}", backup);
            if let Some(ref metadata) = backup.metadata {
                println!("    Type: {}", metadata.backup_type);
                println!("    Encrypted: {}", metadata.encrypted);
                println!("    Compressed: {}", metadata.compressed);
            }
        } else {
            println!("  {}", backup);
        }
    }

    Ok(())
}

fn handle_verify(backup_file: &str) -> Result<()> {
    let restore_manager = RestoreManager::new()?;
    let validation = restore_manager.validate_backup(backup_file)?;

    if validation.valid {
        println!("Backup file is valid ({} bytes)", validation.size_bytes);
    } else {
        println!(
            "Backup file is invalid: {}",
            validation.error.unwrap_or_default()
        );
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse_args();
    init_logging(cli.verbose);

    let result = match &cli.command {
        cli::Commands::Backup {
            output,
            apk,
            compress,
            password: _,
            all,
        } => handle_backup(&cli, output, apk, compress, all),
        cli::Commands::Restore {
            backup_file,
            password,
        } => handle_restore(&cli, backup_file, password),
        cli::Commands::Devices { detailed } => handle_devices(detailed),
        cli::Commands::List {
            directory,
            detailed,
        } => handle_list(directory, detailed),
        cli::Commands::Verify { backup_file } => handle_verify(backup_file),
        cli::Commands::InstallAdb { install_dir: _ } => {
            println!("ADB auto-installation not yet implemented");
            println!("Please install ADB manually: https://developer.android.com/studio/releases/platform-tools");
            Ok(())
        }
    };

    if let Err(e) = result {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}
