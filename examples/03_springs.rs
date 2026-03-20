use blinc_animation::SpringConfig;
use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Spring Animations".to_string(),
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
            .flex_center()
            .gap(32.0)
            .child(animated_button("Snappy", SpringConfig::snappy()))
            .child(animated_button("Gentle", SpringConfig::gentle()))
            .child(animated_button("Wobbly", SpringConfig::wobbly()))
    })
}

fn animated_button(label: &str, spring: SpringConfig) -> impl ElementBuilder {
    stateful::<ButtonState>().on_state(move |ctx| {
        let target = match ctx.state() {
            ButtonState::Hovered => 1.08,
            ButtonState::Pressed => 0.95,
            _ => 1.0,
        };
        let scale = ctx.use_spring("scale", target, spring);

        div()
            .bg(Color::rgba(0.3, 0.5, 0.9, 1.0))
            .rounded(12.0)
            .p(20.0)
            .scale(scale)
            .child(
                text(label)
                    .size(20.0)
                    .weight(FontWeight::SemiBold)
                    .color(Color::WHITE),
            )
    })
}
