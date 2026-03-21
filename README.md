# android-backup

A fast, reliable command-line tool for backing up and restoring Android devices using the Android Debug Bridge (ADB).

## Features

- **Full Backup & Restore** — Create complete backups of your Android device data via ADB
- **Graphical UI** — Modern desktop interface with Blinc framework (optional)
- **AES-256-GCM Encryption** — Secure your backups with strong encryption
- **APK Inclusion** — Optionally include installed APKs in your backups
- **Multi-Device Support** — Work with multiple devices using serial numbers
- **Integrity Verification** — Validate backup files before restoring
- **Compression Support** — Reduce backup size with gzip compression

## Requirements

- [Android SDK Platform Tools](https://developer.android.com/studio/releases/platform-tools) (ADB must be in your PATH, or use `--adb-path`)
- A Linux, macOS, or Windows system
- An Android device with USB debugging enabled

## Installation

### From Source

```bash
cargo build --release
```

The binary will be at `target/release/android-backup`.

### Pre-built Binaries

Download pre-built binaries from the releases page.

## Quick Start

### List Connected Devices

```bash
android-backup devices
```

### Create a Backup

```bash
android-backup backup --output ./my-backup.ab
```

Create an encrypted backup:

```bash
android-backup backup --output ./my-backup.ab --password my-secret-password
```

Include APKs in the backup:

```bash
android-backup backup --output ./my-backup.ab --apk
```

### Restore a Backup

```bash
android-backup restore ./my-backup.ab
```

### Verify Backup Integrity

```bash
android-backup verify ./my-backup.ab
```

### List Local Backups

```bash
android-backup list ~/.local/share/android-backup/backups/
```

## CLI Reference

```text
android-backup [OPTIONS] <COMMAND>
```

### Global Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Enable verbose output |
| `-a, --adb-path <PATH>` | Path to ADB binary |
| `-s, --serial <SERIAL>` | Device serial number |

### Commands

#### `backup`

Create a new backup.

```bash
android-backup backup [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-o, --output <PATH>` | Output file path |
| `--apk` | Include APK files |
| `-z, --compress` | Compress backup (default: true) |
| `-p, --password <PWD>` | Encryption password |
| `--all` | Include all data (default: true) |

#### `restore`

Restore from a backup file.

```bash
android-backup restore <BACKUP_FILE>
```

#### `devices`

List connected devices.

```bash
android-backup devices [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-w, --wait` | Wait for device |
| `-t, --timeout <SECONDS>` | Wait timeout |

#### `list`

List backups in a directory.

```bash
android-backup list [DIRECTORY]
```

#### `verify`

Verify backup file integrity.

```bash
android-backup verify <BACKUP_FILE>
```

## Graphical UI

Android Backup Tool includes an optional graphical user interface built with [Blinc](https://github.com/nickel-lang/blinc):

```bash
cargo run --features blinc-ui -- ui
```

The UI provides:

- **Devices View** — Connect and manage Android devices with USB debugging
- **Backup View** — Configure and run backups with options for APKs and compression
- **Restore View** — Browse local backups and restore to your device
- **Progress Indicators** — Visual feedback during backup and restore operations

### UI Requirements

- Linux (with GPU support) or Windows
- Note: macOS ARM64 (Apple Silicon) is not currently supported due to Blinc framework limitations

## How It Works

android-backup uses ADB to create Android backup archives (`.ab` files). These are tar archives compressed with gzip and optionally encrypted with AES-256-GCM.

The tool stores metadata alongside each backup (`.ab_metadata.json`) containing:

- Backup creation timestamp
- Device information
- Encryption details
- Checksums for integrity verification

## License

MIT
