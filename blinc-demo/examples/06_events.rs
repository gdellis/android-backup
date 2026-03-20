use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Event Handling".to_string(),
        width: 600,
        height: 400,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, |ctx| {
        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::rgba(0.1, 0.1, 0.15, 1.0))
            .flex_col()
            .items_center()
            .justify_center()
            .gap(24.0)
            .child(
                div()
                    .bg(Color::rgba(0.2, 0.3, 0.5, 1.0))
                    .rounded(16.0)
                    .p(32.0)
                    .on_click(|ctx| {
                        println!("Clicked at ({:.0}, {:.0})", ctx.local_x, ctx.local_y);
                    })
                    .on_hover_enter(|_| {
                        println!("Mouse entered");
                    })
                    .on_hover_leave(|_| {
                        println!("Mouse left");
                    })
                    .child(
                        text("Click or Hover Me")
                            .size(20.0)
                            .weight(FontWeight::SemiBold)
                            .color(Color::WHITE),
                    )
                    .child(
                        text("Check console for events")
                            .size(12.0)
                            .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                    ),
            )
            .child(
                div()
                    .bg(Color::rgba(0.3, 0.2, 0.4, 1.0))
                    .rounded(16.0)
                    .p(32.0)
                    .on_key_down(|ctx| {
                        if ctx.key_code == 13 {
                            println!("Enter pressed!");
                        } else {
                            println!("Key pressed: {}", ctx.key_code);
                        }
                    })
                    .child(
                        text("Click and Press Keys")
                            .size(20.0)
                            .weight(FontWeight::SemiBold)
                            .color(Color::WHITE),
                    )
                    .child(
                        text("Focus and press any key")
                            .size(12.0)
                            .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                    ),
            )
    })
}
