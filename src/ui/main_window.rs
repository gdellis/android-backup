use crate::adb::{AdbDevice, DeviceState};
use crate::device::DeviceManager;
use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};
use blinc_core::Color;

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
            #empty-state {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                flex: 1;
                gap: 16px;
                color: #64748b;
            }
            #empty-icon {
                font-size: 64px;
            }
            #error-banner {
                background: #7f1d1d;
                color: #fca5a5;
                padding: 12px 16px;
                border-radius: 8px;
                font-size: 14px;
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

    let mut device_cards = div().id("device-list").flex_col();
    for device in device_list {
        let is_selected = selected.as_deref() == Some(&device.serial);
        let serial = device.serial.clone();
        let model = device
            .model
            .clone()
            .unwrap_or_else(|| "Unknown".to_string());
        let state = device.state;
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

    let content = match current {
        NavItem::Devices => div()
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
            .child(device_cards),
        NavItem::Backup => div()
            .flex_col()
            .items_center()
            .justify_center()
            .child(text("💾 Backup View").size(24.0).color(Color::WHITE))
            .child(
                text("Configure and run backups")
                    .size(14.0)
                    .color(Color::rgb(0.5, 0.5, 0.5)),
            ),
        NavItem::Restore => div()
            .flex_col()
            .items_center()
            .justify_center()
            .child(text("🔄 Restore View").size(24.0).color(Color::WHITE))
            .child(
                text("Restore from backup files")
                    .size(14.0)
                    .color(Color::rgb(0.5, 0.5, 0.5)),
            ),
        NavItem::Settings => div()
            .flex_col()
            .items_center()
            .justify_center()
            .child(text("⚙️ Settings View").size(24.0).color(Color::WHITE))
            .child(
                text("Configure application settings")
                    .size(14.0)
                    .color(Color::rgb(0.5, 0.5, 0.5)),
            ),
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
