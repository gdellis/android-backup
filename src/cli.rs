use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "android-backup",
    about = "Backup and restore your Android device using ADB",
    version = "0.1.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, help = "Enable verbose output")]
    pub verbose: bool,

    #[arg(short, long, global = true, help = "ADB binary path")]
    pub adb_path: Option<PathBuf>,

    #[arg(short, long, global = true, help = "Device serial number")]
    pub serial: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Backup {
        #[arg(short, long, help = "Output backup file path")]
        output: Option<String>,

        #[arg(short, long, help = "Include APK files")]
        apk: bool,

        #[arg(short = 'z', long, help = "Compress backup")]
        compress: bool,

        #[arg(short, long, help = "Encryption password")]
        password: Option<String>,

        #[arg(long, help = "Include all data")]
        all: bool,
    },
    Restore {
        #[arg(help = "Backup file to restore")]
        backup_file: String,

        #[arg(short, long, help = "Decryption password")]
        password: Option<String>,
    },
    Devices {
        #[arg(short, long, help = "Show detailed device info")]
        detailed: bool,
    },
    List {
        #[arg(default_value = ".", help = "Directory to list backups from")]
        directory: String,

        #[arg(short, long, help = "Show detailed metadata")]
        detailed: bool,
    },
    Verify {
        #[arg(help = "Backup file to verify")]
        backup_file: String,
    },
    InstallAdb {
        #[arg(short, long, help = "Install directory")]
        install_dir: Option<PathBuf>,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
