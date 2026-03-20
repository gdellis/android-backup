use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - CSS Transitions".to_string(),
        width: 600,
        height: 450,
        resizable: true,
        ..Default::default()
    };

    let mut css_loaded = false;

    WindowedApp::run(config, move |ctx| {
        if !css_loaded {
            ctx.add_css(
                r#"
                .card {
                    background: #1e3a5f;
                    border-radius: 16px;
                    padding: 24px;
                    transition: all 300ms ease;
                }
                .card:hover {
                    background: #3b82f6;
                    transform: translateY(-8px) scale(1.02);
                    box-shadow: 0 12px 40px rgba(59, 130, 246, 0.5);
                }
                .btn {
                    background: #10b981;
                    border-radius: 8px;
                    padding: 14px 28px;
                    cursor: pointer;
                    transition: all 200ms ease;
                }
                .btn:hover {
                    background: #059669;
                    transform: scale(1.05);
                }
                .btn:active {
                    transform: scale(0.95);
                }
                .icon-box {
                    width: 80px;
                    height: 80px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    border-radius: 16px;
                    transition: all 300ms ease;
                }
                .icon-box:hover {
                    transform: rotate(15deg) scale(1.1);
                    box-shadow: 0 8px 30px rgba(102, 126, 234, 0.6);
                }
                "#,
            );
            css_loaded = true;
        }

        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::rgba(0.05, 0.05, 0.1, 1.0))
            .flex_col()
            .items_center()
            .justify_center()
            .gap(32.0)
            .child(
                div()
                    .class("card")
                    .flex_col()
                    .items_center()
                    .gap(12.0)
                    .child(
                        text("Hover Me!")
                            .size(22.0)
                            .weight(FontWeight::SemiBold)
                            .color(Color::WHITE),
                    )
                    .child(
                        text("CSS transitions handle the animation")
                            .size(14.0)
                            .color(Color::rgba(0.7, 0.7, 0.7, 1.0)),
                    ),
            )
            .child(
                div()
                    .flex_row()
                    .gap(24.0)
                    .child(
                        div().class("btn").child(
                            text("Button")
                                .size(16.0)
                                .weight(FontWeight::SemiBold)
                                .color(Color::WHITE),
                        ),
                    )
                    .child(
                        div().class("btn").child(
                            text("Hover")
                                .size(16.0)
                                .weight(FontWeight::SemiBold)
                                .color(Color::WHITE),
                        ),
                    ),
            )
            .child(
                div()
                    .class("icon-box")
                    .flex_center()
                    .child(text("Icon").size(14.0).color(Color::WHITE)),
            )
    })
}
