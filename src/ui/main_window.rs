use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};
use blinc_core::Color;

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
    static CSS_LOADED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

    CSS_LOADED.get_or_init(|| {
        ctx.add_css(
            r#"
            #root {
                background: #0f172a;
                width: 100%;
                height: 100%;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
            }
            "#,
        );
    });

    div()
        .w(ctx.width)
        .h(ctx.height)
        .id("root")
        .flex_col()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(100.0)
                .h(100.0)
                .bg(Color::rgb(59.0 / 255.0, 130.0 / 255.0, 246.0 / 255.0))
                .rounded(12.0),
        )
        .child(text("Android Backup Tool").size(32.0).color(Color::WHITE))
}
