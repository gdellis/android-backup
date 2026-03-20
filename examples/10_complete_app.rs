use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Complete App".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        ..Default::default()
    };

    let mut css_loaded = false;

    WindowedApp::run(config, move |ctx| {
        if !css_loaded {
            ctx.add_css(
                r#"
                :root {
                    --bg-dark: #0f172a;
                    --bg-card: #1e293b;
                    --text-primary: #f8fafc;
                    --accent: #3b82f6;
                }
                .app-card {
                    background: var(--bg-card);
                    border-radius: 20px;
                    padding: 32px;
                    transition: all 200ms ease;
                }
                .app-stat {
                    background: var(--bg-card);
                    border-radius: 12px;
                    padding: 20px;
                    flex: 1;
                }
                .app-btn-primary {
                    background: var(--accent);
                    border-radius: 12px;
                    padding: 16px 32px;
                    cursor: pointer;
                    transition: all 200ms ease;
                }
                "#,
            );
            css_loaded = true;
        }

        let count = ctx.use_state_keyed("app_counter", || 0i32);

        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::from_hex("#0f172a").unwrap())
            .flex_center()
            .child(
                div()
                    .glass()
                    .rounded(32.0)
                    .p(48.0)
                    .flex_col()
                    .items_center()
                    .gap(24.0)
                    .child(
                        text("Complete App Demo")
                            .size(32.0)
                            .weight(FontWeight::Bold)
                            .color(Color::WHITE),
                    )
                    .child(
                        div()
                            .flex_row()
                            .gap(16.0)
                            .child(app_stat(&count, "Backups"))
                            .child(app_stat_static("Size", "2.4 GB"))
                            .child(app_stat_static("Devices", "3")),
                    )
                    .child(
                        div()
                            .flex_row()
                            .gap(16.0)
                            .child(inc_button(&count, "-", -1))
                            .child(inc_button(&count, "+", 1)),
                    )
                    .child(
                        div()
                            .class("app-btn-primary")
                            .p(16.0)
                            .flex_center()
                            .on_click(|_| println!("Create Backup clicked!"))
                            .child(
                                text("Create Backup")
                                    .size(16.0)
                                    .weight(FontWeight::SemiBold)
                                    .color(Color::WHITE),
                            ),
                    ),
            )
    })
}

fn app_stat(count: &State<i32>, label: &str) -> impl ElementBuilder {
    div()
        .class("app-stat")
        .flex_col()
        .items_center()
        .gap(8.0)
        .child(
            text(label)
                .size(14.0)
                .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
        )
        .child(
            stateful::<NoState>()
                .deps([count.signal_id()])
                .on_state(move |_| {
                    text(&format!("{}", count.get()))
                        .size(28.0)
                        .weight(FontWeight::Bold)
                        .color(Color::WHITE)
                }),
        )
}

fn app_stat_static(label: &str, value: &str) -> impl ElementBuilder {
    div()
        .class("app-stat")
        .flex_col()
        .items_center()
        .gap(8.0)
        .child(
            text(label)
                .size(14.0)
                .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
        )
        .child(
            text(value)
                .size(28.0)
                .weight(FontWeight::Bold)
                .color(Color::WHITE),
        )
}

fn inc_button(count: &State<i32>, label: &str, delta: i32) -> impl ElementBuilder {
    let bg = if delta > 0 {
        Color::rgba(0.3, 0.8, 0.3, 1.0)
    } else {
        Color::rgba(0.8, 0.3, 0.3, 1.0)
    };
    div()
        .bg(bg)
        .rounded(12.0)
        .w(56.0)
        .h(56.0)
        .flex_center()
        .on_click(move |_| count.update(|v| v + delta))
        .child(
            text(label)
                .size(28.0)
                .weight(FontWeight::Bold)
                .color(Color::WHITE),
        )
}
