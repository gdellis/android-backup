use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Flexbox Layout".to_string(),
        width: 600,
        height: 400,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, |ctx| {
        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::rgba(0.1, 0.1, 0.2, 1.0))
            .flex_col()
            .gap(16.0)
            .p(24.0)
            .child(
                text("Flexbox Layout Demo")
                    .size(28.0)
                    .weight(FontWeight::Bold)
                    .color(Color::WHITE),
            )
            .child(
                div()
                    .flex_row()
                    .gap(12.0)
                    .p(16.0)
                    .bg(Color::rgba(0.2, 0.3, 0.5, 1.0))
                    .rounded(12.0)
                    .child(text("Item 1").size(18.0).color(Color::WHITE))
                    .child(text("Item 2").size(18.0).color(Color::WHITE))
                    .child(text("Item 3").size(18.0).color(Color::WHITE)),
            )
            .child(
                div()
                    .items_center()
                    .justify_center()
                    .w(300.0)
                    .h(120.0)
                    .bg(Color::rgba(0.3, 0.2, 0.4, 1.0))
                    .rounded(16.0)
                    .child(text("Centered Content").size(20.0).color(Color::WHITE)),
            )
            .child(
                div()
                    .flex_row()
                    .justify_end()
                    .gap(8.0)
                    .p(16.0)
                    .bg(Color::rgba(0.2, 0.4, 0.3, 1.0))
                    .rounded(12.0)
                    .child(text("Right").size(16.0).color(Color::WHITE))
                    .child(text("Aligned").size(16.0).color(Color::WHITE)),
            )
    })
}
