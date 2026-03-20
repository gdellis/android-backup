use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Dark Theme".to_string(),
        width: 700,
        height: 500,
        resizable: true,
        ..Default::default()
    };

    let mut css_loaded = false;

    WindowedApp::run(config, move |ctx| {
        if !css_loaded {
            ctx.add_css(
                r#"
                :root {
                    --bg-primary: #0f172a;
                    --bg-secondary: #1e293b;
                    --bg-tertiary: #334155;
                    --text-primary: #f8fafc;
                    --text-secondary: #94a3b8;
                    --accent: #3b82f6;
                    --accent-hover: #2563eb;
                    --success: #10b981;
                }
                .theme-page {
                    background: var(--bg-primary);
                }
                .theme-card {
                    background: var(--bg-secondary);
                    border-radius: 16px;
                    padding: 24px;
                    transition: all 200ms ease;
                }
                .theme-card:hover {
                    background: var(--bg-tertiary);
                    transform: translateY(-2px);
                }
                .theme-title {
                    color: var(--text-primary);
                    font-size: 24px;
                    font-weight: bold;
                }
                .theme-subtitle {
                    color: var(--text-secondary);
                    font-size: 14px;
                }
                .theme-btn {
                    background: var(--accent);
                    border-radius: 8px;
                    padding: 12px 24px;
                    cursor: pointer;
                    transition: all 200ms ease;
                }
                .theme-btn:hover {
                    background: var(--accent-hover);
                    transform: scale(1.02);
                }
                .theme-badge {
                    background: var(--success);
                    border-radius: 20px;
                    padding: 4px 12px;
                    font-size: 12px;
                    color: white;
                }
                .theme-divider {
                    height: 1px;
                    background: var(--bg-tertiary);
                    margin: 16px 0;
                }
                "#,
            );
            css_loaded = true;
        }

        div()
            .w(ctx.width)
            .h(ctx.height)
            .class("theme-page")
            .flex_col()
            .items_center()
            .p(40.0)
            .gap(24.0)
            .child(text("Dark Theme Demo").class("theme-title"))
            .child(
                div()
                    .flex_row()
                    .gap(20.0)
                    .flex_wrap()
                    .child(
                        div()
                            .class("theme-card")
                            .flex_col()
                            .gap(12.0)
                            .w(280.0)
                            .child(
                                div()
                                    .flex_row()
                                    .justify_between()
                                    .child(text("Statistics").class("theme-title").size(18.0))
                                    .child(div().class("theme-badge").child(text("Live").size(11.0))),
                            )
                            .child(div().class("theme-divider"))
                            .child(
                                div()
                                    .flex_col()
                                    .gap(8.0)
                                    .child(theme_stat_row("Users", "1,234"))
                                    .child(theme_stat_row("Revenue", "$45,678"))
                                    .child(theme_stat_row("Growth", "+12.5%")),
                            ),
                    )
                    .child(
                        div()
                            .class("theme-card")
                            .flex_col()
                            .gap(12.0)
                            .w(280.0)
                            .child(text("Actions").class("theme-title").size(18.0))
                            .child(div().class("theme-divider"))
                            .child(
                                div()
                                    .flex_col()
                                    .gap(8.0)
                                    .child(
                                        div()
                                            .class("theme-btn")
                                            .items_center()
            .justify_center()
                                            .child(text("Save Changes").size(14.0).weight(FontWeight::SemiBold).color(Color::WHITE)),
                                    )
                                    .child(
                                        div()
                                            .items_center()
            .justify_center()
                                            .p(12.0)
                                            .child(text("Cancel").size(14.0).color(Color::rgba(0.6, 0.6, 0.6, 1.0))),
                                    ),
                            ),
                    ),
            )
                            .child(div().class("theme-divider"))
                            .child(
                                div()
                                    .flex_col()
                                    .gap(8.0)
                                    .child(theme_stat_row("Users", "1,234"))
                                    .child(theme_stat_row("Revenue", "$45,678"))
                                    .child(theme_stat_row("Growth", "+12.5%")),
                            ),
                    )
                    .child(
                        div()
                            .class("theme-card")
                            .flex_col()
                            .gap(12.0)
                            .w(280.0)
                            .child(text("Actions").class("theme-title").size(18.0))
                            .child(div().class("theme-divider"))
                            .child(
                                div()
                                    .flex_col()
                                    .gap(8.0)
                                    .child(
                                        div().class("theme-btn").items_center().justify_center().child(
                                            text("Save Changes")
                                                .size(14.0)
                                                .weight(FontWeight::SemiBold)
                                                .color(Color::WHITE),
                                        ),
                                    )
                                    .child(
                                        div().items_center().justify_center().p(12.0).child(
                                            text("Cancel")
                                                .size(14.0)
                                                .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                                        ),
                                    ),
                            ),
                    ),
            )
    })
}

fn theme_stat_row(label: &str, value: &str) -> impl ElementBuilder {
    div()
        .flex_row()
        .justify_between()
        .child(text(label).size(14.0).color(Color::rgba(0.6, 0.6, 0.6, 1.0)))
        .child(text(value).size(14.0).weight(FontWeight::SemiBold).color(Color::WHITE)))
}
