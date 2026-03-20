use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Glass Material".to_string(),
        width: 600,
        height: 500,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, |ctx| {
        div()
            .w(ctx.width)
            .h(ctx.height)
            .items_center()
            .justify_center()
            .bg(Color::rgba(0.05, 0.1, 0.2, 1.0))
            .child(
                div()
                    .glass()
                    .rounded(32.0)
                    .p(48.0)
                    .flex_col()
                    .items_center()
                    .gap(16.0)
                    .child(
                        text("Frosted Glass")
                            .size(32.0)
                            .weight(FontWeight::Bold)
                            .color(Color::WHITE),
                    )
                    .child(
                        text("Glass material creates")
                            .size(16.0)
                            .color(Color::rgba(0.8, 0.8, 0.8, 1.0)),
                    )
                    .child(
                        text("a blur effect behind")
                            .size(16.0)
                            .color(Color::rgba(0.8, 0.8, 0.8, 1.0)),
                    ),
            )
    })
}
