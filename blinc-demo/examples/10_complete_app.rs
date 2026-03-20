use blinc_app::prelude::*;
use blinc_app::windowed::WindowedApp;

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
                .inc-btn {
                    border-radius: 12px;
                    width: 56px;
                    height: 56px;
                }
                .dec-btn {
                    background: rgba(0.8, 0.3, 0.3, 1.0);
                }
                .inc-btn-actual {
                    background: rgba(0.3, 0.8, 0.3, 1.0);
                }
                "#,
            );
            css_loaded = true;
        }

        let count = ctx.use_state_keyed("app_counter", || 0i32);
        let count_for_display = count.clone();

        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::rgba(0.06, 0.09, 0.16, 1.0))
            .flex_col()
            .items_center()
            .justify_center()
            .gap(32.0)
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
                            .child(
                                div()
                                    .class("app-stat")
                                    .flex_col()
                                    .items_center()
                                    .gap(8.0)
                                    .child(
                                        text("Backups")
                                            .size(14.0)
                                            .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                                    )
                                    .child(
                                        stateful::<NoState>()
                                            .deps([count_for_display.signal_id()])
                                            .on_state(move |_| {
                                                div().child(
                                                    text(&count_for_display.get().to_string())
                                                        .size(28.0)
                                                        .weight(FontWeight::Bold)
                                                        .color(Color::WHITE),
                                                )
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .class("app-stat")
                                    .flex_col()
                                    .items_center()
                                    .gap(8.0)
                                    .child(
                                        text("Size")
                                            .size(14.0)
                                            .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                                    )
                                    .child(
                                        text("2.4 GB")
                                            .size(28.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE),
                                    ),
                            )
                            .child(
                                div()
                                    .class("app-stat")
                                    .flex_col()
                                    .items_center()
                                    .gap(8.0)
                                    .child(
                                        text("Devices")
                                            .size(14.0)
                                            .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                                    )
                                    .child(
                                        text("3")
                                            .size(28.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex_row()
                            .gap(16.0)
                            .child(
                                div()
                                    .class("inc-btn dec-btn")
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .on_click({
                                        let c = count.clone();
                                        move |_| {
                                            let val = c.get();
                                            c.set(val - 1);
                                        }
                                    })
                                    .child(
                                        text("-")
                                            .size(28.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE),
                                    ),
                            )
                            .child(
                                div()
                                    .class("inc-btn inc-btn-actual")
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .on_click({
                                        let c = count.clone();
                                        move |_| {
                                            let val = c.get();
                                            c.set(val + 1);
                                        }
                                    })
                                    .child(
                                        text("+")
                                            .size(28.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .class("app-btn-primary")
                            .p(16.0)
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
