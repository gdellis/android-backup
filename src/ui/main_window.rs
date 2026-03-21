use crate::adb::{AdbDevice, DeviceState};
use crate::backup::BackupManager;
use crate::device::DeviceManager;
use crate::restore::RestoreManager;
use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};
use blinc_core::Color;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BackupListItem {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub created_at: String,
    pub backup_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavItem {
    #[default]
    Devices,
    Backup,
    Restore,
    Settings,
}

pub fn run_ui() -> Result<()> {
    let config = WindowConfig {
        title: "Android Backup Tool".to_string(),
        width: 900,
        height: 700,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, build_ui)
}

fn build_ui(ctx: &mut WindowedContext) -> impl ElementBuilder {
    let selected_nav = ctx.use_state_keyed("selected_nav", || NavItem::Devices);
    let devices = ctx.use_state_keyed::<Vec<AdbDevice>, _>("devices", Vec::new);
    let selected_serial = ctx.use_state_keyed::<Option<String>, _>("selected_serial", || None);
    let is_loading = ctx.use_state_keyed("is_loading", || false);
    let error_msg = ctx.use_state_keyed::<Option<String>, _>("error_msg", || None);

    let backup_apk = ctx.use_state_keyed("backup_apk", || false);
    let backup_compress = ctx.use_state_keyed("backup_compress", || true);
    let backup_output = ctx.use_state_keyed("backup_output", get_default_backup_dir);
    let is_backuping = ctx.use_state_keyed("is_backuping", || false);
    let backup_success = ctx.use_state_keyed("backup_success", || false);
    let backup_error = ctx.use_state_keyed::<Option<String>, _>("backup_error", || None);

    let restore_directory = ctx.use_state_keyed("restore_directory", get_default_backup_dir);
    let restore_files = ctx.use_state_keyed::<Vec<BackupListItem>, _>("restore_files", Vec::new);
    let selected_backup = ctx.use_state_keyed::<Option<String>, _>("selected_backup", || None);
    let is_restoring = ctx.use_state_keyed("is_restoring", || false);
    let restore_success = ctx.use_state_keyed("restore_success", || false);
    let restore_error = ctx.use_state_keyed::<Option<String>, _>("restore_error", || None);

    static CSS_LOADED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

    CSS_LOADED.get_or_init(|| {
        ctx.add_css(
            r#"
            #root {
                background: #0f172a;
                width: 100%;
                height: 100%;
                display: flex;
                flex-direction: row;
            }
            #sidebar {
                width: 220px;
                background: #1e293b;
                height: 100%;
                display: flex;
                flex-direction: column;
                padding: 16px 12px;
            }
            #sidebar-title {
                font-size: 18px;
                font-weight: 700;
                color: #f8fafc;
                padding: 8px 12px;
                margin-bottom: 16px;
            }
            #nav-items {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .nav-item {
                display: flex;
                flex-direction: row;
                align-items: center;
                gap: 12px;
                padding: 12px 16px;
                border-radius: 8px;
                cursor: pointer;
                color: #94a3b8;
                font-size: 14px;
            }
            .nav-item:hover {
                background: #334155;
                color: #f8fafc;
            }
            .nav-item.selected {
                background: #3b82f6;
                color: #ffffff;
            }
            #content {
                flex: 1;
                display: flex;
                flex-direction: column;
                overflow: hidden;
            }
            #devices-view {
                width: 100%;
                height: 100%;
                display: flex;
                flex-direction: column;
                padding: 24px;
                gap: 20px;
            }
            #devices-header {
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                align-items: center;
            }
            #devices-title {
                font-size: 24px;
                font-weight: 700;
                color: #f8fafc;
            }
            #refresh-btn {
                padding: 10px 20px;
                background: #3b82f6;
                border-radius: 8px;
                color: white;
                font-size: 14px;
                cursor: pointer;
            }
            #refresh-btn:hover {
                background: #2563eb;
            }
            #device-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
                flex: 1;
                overflow-y: auto;
            }
            .device-card {
                background: #1e293b;
                border-radius: 12px;
                padding: 16px 20px;
                cursor: pointer;
                border: 2px solid transparent;
            }
            .device-card:hover {
                border-color: #475569;
            }
            .device-card.selected {
                border-color: #3b82f6;
                background: #1e3a5f;
            }
            #error-banner {
                background: #7f1d1d;
                color: #fca5a5;
                padding: 12px 16px;
                border-radius: 8px;
                font-size: 14px;
            }
            #backup-view {
                width: 100%;
                height: 100%;
                display: flex;
                flex-direction: column;
                padding: 24px;
                gap: 24px;
                overflow-y: auto;
            }
            #backup-header {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            #backup-title {
                font-size: 24px;
                font-weight: 700;
                color: #f8fafc;
            }
            #backup-subtitle {
                font-size: 14px;
                color: #64748b;
            }
            .config-section {
                background: #1e293b;
                border-radius: 12px;
                padding: 20px;
                display: flex;
                flex-direction: column;
                gap: 16px;
            }
            .section-title {
                font-size: 16px;
                font-weight: 600;
                color: #f8fafc;
            }
            .config-row {
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-between;
            }
            .config-label {
                font-size: 14px;
                color: #e2e8f0;
            }
            .config-desc {
                font-size: 12px;
                color: #64748b;
                margin-top: 4px;
            }
            .toggle {
                width: 48px;
                height: 24px;
                background: #475569;
                border-radius: 12px;
                cursor: pointer;
                position: relative;
            }
            .toggle.active {
                background: #3b82f6;
            }
            .toggle-knob {
                width: 20px;
                height: 20px;
                background: white;
                border-radius: 50%;
                position: absolute;
                top: 2px;
                left: 2px;
                transition: left 150ms ease;
            }
            .toggle.active .toggle-knob {
                left: 26px;
            }
            #start-backup-btn {
                padding: 14px 24px;
                background: #3b82f6;
                border-radius: 8px;
                color: white;
                font-size: 16px;
                font-weight: 600;
                cursor: pointer;
                text-align: center;
            }
            #start-backup-btn:hover {
                background: #2563eb;
            }
            .placeholder-card {
                background: #334155;
                border-radius: 8px;
                padding: 12px 16px;
                color: #94a3b8;
                font-size: 14px;
                text-align: center;
            }
            #restore-view {
                width: 100%;
                height: 100%;
                display: flex;
                flex-direction: column;
                padding: 24px;
                gap: 24px;
                overflow-y: auto;
            }
            #restore-header {
                display: flex;
                flex-direction: column;
                gap: 8px;
            }
            #restore-title {
                font-size: 24px;
                font-weight: 700;
                color: #f8fafc;
            }
            #restore-subtitle {
                font-size: 14px;
                color: #64748b;
            }
            .backup-list {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .backup-item {
                background: #1e293b;
                border-radius: 12px;
                padding: 16px 20px;
                cursor: pointer;
                border: 2px solid transparent;
            }
            .backup-item:hover {
                border-color: #475569;
            }
            .backup-item.selected {
                border-color: #3b82f6;
                background: #1e3a5f;
            }
            .backup-item-name {
                font-size: 16px;
                font-weight: 600;
                color: #f8fafc;
                margin-bottom: 4px;
            }
            .backup-item-meta {
                font-size: 12px;
                color: #64748b;
                display: flex;
                gap: 16px;
            }
            #start-restore-btn {
                padding: 14px 24px;
                background: #3b82f6;
                border-radius: 8px;
                color: white;
                font-size: 16px;
                font-weight: 600;
                cursor: pointer;
                text-align: center;
            }
            #start-restore-btn:hover {
                background: #2563eb;
            }
            #success-banner {
                background: #166534;
                color: #86efac;
                padding: 16px 20px;
                border-radius: 8px;
                font-size: 14px;
            }
            .progress-container {
                background: #1e293b;
                border-radius: 12px;
                padding: 20px;
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .progress-label {
                font-size: 14px;
                color: #e2e8f0;
                font-weight: 500;
            }
            .progress-bar {
                background: #334155;
                height: 8px;
                border-radius: 4px;
                overflow: hidden;
            }
            .progress-fill {
                background: #3b82f6;
                height: 100%;
                border-radius: 4px;
                transition: width 300ms ease;
            }
            .progress-fill.complete {
                background: #22c55e;
            }
            "#,
        );
    });

    let current = selected_nav.get();

    let devices_click = {
        let nav = selected_nav.clone();
        move || nav.set(NavItem::Devices)
    };
    let backup_click = {
        let nav = selected_nav.clone();
        move || nav.set(NavItem::Backup)
    };
    let restore_click = {
        let nav = selected_nav.clone();
        move || nav.set(NavItem::Restore)
    };
    let settings_click = {
        let nav = selected_nav.clone();
        move || nav.set(NavItem::Settings)
    };

    let refresh_click = {
        let devices = devices.clone();
        let is_loading = is_loading.clone();
        let error_msg = error_msg.clone();
        move || {
            is_loading.set(true);
            error_msg.set(None);
            match DeviceManager::new() {
                Ok(manager) => match manager.list_devices() {
                    Ok(list) => {
                        devices.set(list);
                        is_loading.set(false);
                    }
                    Err(e) => {
                        error_msg.set(Some(e.to_string()));
                        is_loading.set(false);
                    }
                },
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    is_loading.set(false);
                }
            }
        }
    };

    let loading = is_loading.get();
    let error = error_msg.get();
    let device_list = devices.get();
    let selected = selected_serial.get();
    let apk = backup_apk.get();
    let compress = backup_compress.get();
    let output_dir = backup_output.get();
    let is_backuping_val = is_backuping.get();

    let mut device_cards = div().id("device-list").flex_col();
    for device in &device_list {
        let is_selected = selected.as_deref() == Some(&device.serial);
        let serial = device.serial.clone();
        let model = device
            .model
            .clone()
            .unwrap_or_else(|| "Unknown".to_string());
        let state = device.state.clone();
        let serial_clone = serial.clone();
        let selected_serial_clone = selected_serial.clone();

        device_cards = device_cards.child(
            div()
                .class(if is_selected {
                    "device-card selected"
                } else {
                    "device-card"
                })
                .on_click(move |_ctx| {
                    selected_serial_clone.set(Some(serial_clone.clone()));
                })
                .flex_col()
                .child(text("📱"))
                .child(text(&model))
                .child(text(&serial))
                .child(text(match state {
                    DeviceState::Device => "Authorized",
                    DeviceState::Unauthorized => "Unauthorized",
                    DeviceState::Offline => "Offline",
                })),
        );
    }

    let devices_content = div()
        .id("devices-view")
        .flex_col()
        .child(
            div()
                .id("devices-header")
                .flex_row()
                .child(text("Connected Devices").id("devices-title"))
                .child(
                    div()
                        .id("refresh-btn")
                        .on_click(move |_ctx| refresh_click())
                        .child(text(if loading { "Loading..." } else { "Refresh" })),
                ),
        )
        .child(
            div()
                .id("error-banner")
                .child(text(error.as_deref().unwrap_or("Unknown error"))),
        )
        .child(device_cards);

    let selected_device_info = if let Some(ref s) = selected {
        let model = device_list
            .iter()
            .find(|d| &d.serial == s)
            .and_then(|d| d.model.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        div()
            .flex_col()
            .child(text("📱 ".to_owned() + &model))
            .child(text(s).color(Color::rgb(0.4, 0.4, 0.4)))
    } else {
        div()
            .class("placeholder-card")
            .child(text("No device selected. Go to Devices to select one."))
    };

    let backup_content = div()
        .id("backup-view")
        .flex_col()
        .child(
            div()
                .id("backup-header")
                .child(text("Create Backup").id("backup-title"))
                .child(
                    text("Configure and start a backup of your Android device")
                        .id("backup-subtitle"),
                ),
        )
        .child(
            div()
                .class("config-section")
                .flex_col()
                .child(div().child(text("Device")))
                .child(selected_device_info),
        )
        .child(
            div()
                .class("config-section")
                .flex_col()
                .child(div().child(text("Output")))
                .child(text(&output_dir).color(Color::rgb(0.4, 0.4, 0.4))),
        )
        .child(
            div()
                .class("config-section")
                .flex_col()
                .child(div().child(text("Options")))
                .child(
                    div()
                        .class("config-row")
                        .child(
                            div()
                                .flex_col()
                                .child(text("Include APKs"))
                                .child(text("Backup APK files for installed apps")),
                        )
                        .child(
                            div()
                                .class(if apk { "toggle active" } else { "toggle" })
                                .on_click({
                                    let apk_state = backup_apk.clone();
                                    move |_ctx| {
                                        let new_val = !apk_state.get();
                                        apk_state.set(new_val);
                                    }
                                })
                                .child(div().class("toggle-knob")),
                        ),
                )
                .child(
                    div()
                        .class("config-row")
                        .child(
                            div()
                                .flex_col()
                                .child(text("Compress"))
                                .child(text("Compress backup data")),
                        )
                        .child(
                            div()
                                .class(if compress { "toggle active" } else { "toggle" })
                                .on_click({
                                    let compress_state = backup_compress.clone();
                                    move |_ctx| {
                                        let new_val = !compress_state.get();
                                        compress_state.set(new_val);
                                    }
                                })
                                .child(div().class("toggle-knob")),
                        ),
                ),
        )
        .child(
            div()
                .class("progress-container")
                .flex_col()
                .child(
                    div()
                        .class("progress-label")
                        .child(text(if is_backuping_val {
                            "Backing up..."
                        } else {
                            "Ready to backup"
                        })),
                )
                .child(
                    div().class("progress-bar").child(
                        div()
                            .class(if is_backuping_val {
                                "progress-fill"
                            } else {
                                "progress-fill complete"
                            })
                            .w(100.0),
                    ),
                ),
        )
        .child(
            div()
                .id("start-backup-btn")
                .on_click({
                    let serial_state = selected_serial.clone();
                    let apk_state = backup_apk.clone();
                    let compress_state = backup_compress.clone();
                    let output_state = backup_output.clone();
                    let backup_error = backup_error.clone();
                    let is_backuping = is_backuping.clone();
                    let backup_success = backup_success.clone();
                    move |_ctx| {
                        let serial = serial_state.get();
                        if serial.is_none() {
                            backup_error.set(Some("Please select a device first".to_string()));
                            return;
                        }
                        let serial = serial.unwrap();
                        let apk = apk_state.get();
                        let compress = compress_state.get();
                        let output = output_state.get();

                        backup_error.set(None);
                        is_backuping.set(true);

                        let backup_error_inner = backup_error.clone();
                        let is_backuping_inner = is_backuping.clone();
                        let backup_success_inner = backup_success.clone();

                        std::thread::spawn(move || {
                            let options = crate::adb::BackupOptions {
                                all: true,
                                apk,
                                compress,
                                password: None,
                            };

                            match BackupManager::with_serial(serial.clone()) {
                                Ok(manager) => match manager.create_backup(&output, &options) {
                                    Ok(_) => {
                                        is_backuping_inner.set(false);
                                        backup_success_inner.set(true);
                                    }
                                    Err(e) => {
                                        is_backuping_inner.set(false);
                                        backup_error_inner.set(Some(e.to_string()));
                                    }
                                },
                                Err(e) => {
                                    is_backuping_inner.set(false);
                                    backup_error_inner.set(Some(e.to_string()));
                                }
                            }
                        });
                    }
                })
                .child(text(if is_backuping_val {
                    "Backing up..."
                } else {
                    "Start Backup"
                })),
        );

    let selected = selected_backup.get();
    let restoring = is_restoring.get();
    let restore_err = restore_error.get();
    let restore_dir = restore_directory.get();

    {
        let restore_files = restore_files.clone();
        let restore_error = restore_error.clone();
        let restore_directory = restore_directory.clone();
        let load_fn = move || {
            restore_error.set(None);
            let dir = restore_directory.get();
            match BackupManager::list_backups(&dir) {
                Ok(backups) => {
                    let items: Vec<BackupListItem> = backups
                        .into_iter()
                        .map(|b| BackupListItem {
                            name: b.name,
                            path: b.path,
                            size_bytes: b.size_bytes,
                            created_at: b.created_at.format("%Y-%m-%d %H:%M").to_string(),
                            backup_type: b
                                .metadata
                                .map(|m| m.backup_type)
                                .unwrap_or_else(|| "unknown".to_string()),
                        })
                        .collect();
                    restore_files.set(items);
                }
                Err(e) => {
                    restore_error.set(Some(e.to_string()));
                }
            }
        };
        load_fn();
    }

    let restore_files_list = restore_files.get();

    let mut backup_items = div().class("backup-list");
    for item in &restore_files_list {
        let is_selected = selected.as_deref() == Some(&item.path);
        let path = item.path.clone();
        let path_clone = path.clone();
        let selected_backup_clone = selected_backup.clone();

        let size_mb = item.size_bytes as f64 / (1024.0 * 1024.0);

        backup_items = backup_items.child(
            div()
                .class(if is_selected {
                    "backup-item selected"
                } else {
                    "backup-item"
                })
                .on_click(move |_ctx| {
                    selected_backup_clone.set(Some(path_clone.clone()));
                })
                .flex_col()
                .child(text(&item.name))
                .child(
                    div()
                        .class("backup-item-meta")
                        .child(text(format!("{:.2} MB", size_mb)))
                        .child(text(&item.created_at))
                        .child(text(&item.backup_type)),
                ),
        );
    }

    let restore_content = div()
        .id("restore-view")
        .flex_col()
        .child(
            div()
                .id("restore-header")
                .child(text("Restore Backup").id("restore-title"))
                .child(text("Select a backup to restore to your device").id("restore-subtitle")),
        )
        .child(
            div()
                .class("config-section")
                .flex_col()
                .child(div().child(text("Backup Location")))
                .child(text(&restore_dir).color(Color::rgb(0.4, 0.4, 0.4))),
        )
        .child(
            div()
                .class("config-section")
                .flex_col()
                .child(div().child(text("Available Backups")))
                .child(
                    div()
                        .class("backup-list")
                        .child(if restore_files_list.is_empty() {
                            div()
                                .class("placeholder-card")
                                .child(text("No backups found. Create a backup first."))
                        } else {
                            backup_items
                        }),
                ),
        )
        .child(
            div()
                .id("error-banner")
                .child(text(restore_err.as_deref().unwrap_or("Unknown error"))),
        )
        .child(
            div()
                .class("progress-container")
                .flex_col()
                .child(div().class("progress-label").child(text(if restoring {
                    "Restoring..."
                } else {
                    "Ready to restore"
                })))
                .child(
                    div().class("progress-bar").child(
                        div()
                            .class(if restoring {
                                "progress-fill"
                            } else {
                                "progress-fill complete"
                            })
                            .w(100.0),
                    ),
                ),
        )
        .child(
            div()
                .id("start-restore-btn")
                .on_click({
                    let selected = selected_backup.clone();
                    let selected_serial = selected_serial.clone();
                    let is_restoring = is_restoring.clone();
                    let restore_success = restore_success.clone();
                    let restore_error = restore_error.clone();
                    move |_ctx| {
                        let backup_path = selected.get();
                        let serial = selected_serial.get();

                        if backup_path.is_none() {
                            restore_error.set(Some("Please select a backup first".to_string()));
                            return;
                        }
                        if serial.is_none() {
                            restore_error.set(Some("Please select a device first".to_string()));
                            return;
                        }

                        let backup_path = backup_path.unwrap();
                        let serial = serial.unwrap();

                        is_restoring.set(true);
                        restore_error.set(None);

                        let is_restoring_inner = is_restoring.clone();
                        let restore_success_inner = restore_success.clone();
                        let restore_error_inner = restore_error.clone();

                        std::thread::spawn(move || {
                            match RestoreManager::with_serial(serial.clone()) {
                                Ok(manager) => match manager.restore(&backup_path, None) {
                                    Ok(_) => {
                                        is_restoring_inner.set(false);
                                        restore_success_inner.set(true);
                                    }
                                    Err(e) => {
                                        is_restoring_inner.set(false);
                                        restore_error_inner.set(Some(e.to_string()));
                                    }
                                },
                                Err(e) => {
                                    is_restoring_inner.set(false);
                                    restore_error_inner.set(Some(e.to_string()));
                                }
                            }
                        });
                    }
                })
                .child(text(if restoring {
                    "Restoring..."
                } else {
                    "Start Restore"
                })),
        );

    let settings_content = div()
        .flex_col()
        .items_center()
        .justify_center()
        .child(text("⚙️ Settings View").size(24.0).color(Color::WHITE))
        .child(
            text("Configure application settings")
                .size(14.0)
                .color(Color::rgb(0.5, 0.5, 0.5)),
        );

    let content = match current {
        NavItem::Devices => devices_content,
        NavItem::Backup => backup_content,
        NavItem::Restore => restore_content,
        NavItem::Settings => settings_content,
    };

    div()
        .id("root")
        .flex_row()
        .child(
            div()
                .id("sidebar")
                .flex_col()
                .child(text("Android Backup").id("sidebar-title"))
                .child(
                    div()
                        .id("nav-items")
                        .flex_col()
                        .child(nav_button(
                            "📱 Devices",
                            current == NavItem::Devices,
                            devices_click,
                        ))
                        .child(nav_button(
                            "💾 Backup",
                            current == NavItem::Backup,
                            backup_click,
                        ))
                        .child(nav_button(
                            "🔄 Restore",
                            current == NavItem::Restore,
                            restore_click,
                        ))
                        .child(nav_button(
                            "⚙️ Settings",
                            current == NavItem::Settings,
                            settings_click,
                        )),
                ),
        )
        .child(div().id("content").child(content))
}

fn nav_button(
    label: &str,
    is_selected: bool,
    on_click: impl Fn() + 'static,
) -> impl ElementBuilder {
    div()
        .class(if is_selected {
            "nav-item selected"
        } else {
            "nav-item"
        })
        .on_click(move |_ctx| on_click())
        .child(text(label))
}

fn get_default_backup_dir() -> String {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("android-backup")
        .join("backups")
        .to_string_lossy()
        .to_string()
}
