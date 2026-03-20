use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavItem {
    #[default]
    Devices,
    Backup,
    Restore,
    Settings,
}

impl NavItem {
    fn label(&self) -> &'static str {
        match self {
            NavItem::Devices => "Devices",
            NavItem::Backup => "Backup",
            NavItem::Restore => "Restore",
            NavItem::Settings => "Settings",
        }
    }
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
                align-items: center;
                justify-content: center;
                color: #94a3b8;
                font-size: 24px;
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
        .child(
            div()
                .id("content")
                .child(text(format!("{} View", current.label()))),
        )
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
