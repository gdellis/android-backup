use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Reactive Counter".to_string(),
        width: 500,
        height: 400,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, |ctx| {
        let count = ctx.use_state_keyed("counter", || 0i32);

        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::rgba(0.1, 0.1, 0.15, 1.0))
            .flex_center()
            .child(
                div()
                    .glass()
                    .rounded(32.0)
                    .p(48.0)
                    .flex_col()
                    .items_center()
                    .gap(32.0)
                    .child(
                        text("Counter Demo")
                            .size(24.0)
                            .weight(FontWeight::Bold)
                            .color(Color::WHITE),
                    )
                    .child(
                        stateful::<NoState>()
                            .deps([count.signal_id()])
                            .on_state(move |_| {
                                let value = count.get();
                                text(&format!("{}", value))
                                    .size(72.0)
                                    .weight(FontWeight::Bold)
                                    .color(Color::WHITE)
                            }),
                    )
                    .child(
                        div()
                            .flex_row()
                            .gap(24.0)
                            .child(counter_btn(
                                &count,
                                "-",
                                -1,
                                Color::rgba(0.8, 0.3, 0.3, 1.0),
                            ))
                            .child(counter_btn(&count, "+", 1, Color::rgba(0.3, 0.8, 0.3, 1.0))),
                    )
                    .child(
                        div()
                            .p(12.0)
                            .on_click(|_| {
                                count.set(0);
                            })
                            .child(
                                text("Reset")
                                    .size(14.0)
                                    .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                            ),
                    ),
            )
    })
}

fn counter_btn(count: &State<i32>, label: &str, delta: i32, bg: Color) -> impl ElementBuilder {
    stateful::<ButtonState>().on_state(move |ctx| {
        let scale = match ctx.state() {
            ButtonState::Hovered => 1.1,
            ButtonState::Pressed => 0.9,
            _ => 1.0,
        };

        div()
            .bg(bg)
            .rounded(12.0)
            .w(64.0)
            .h(64.0)
            .flex_center()
            .scale(scale)
            .on_click(move |_| {
                count.update(|v| v + delta);
            })
            .child(
                text(label)
                    .size(32.0)
                    .weight(FontWeight::Bold)
                    .color(Color::WHITE),
            )
    })
}
