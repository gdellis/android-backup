use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Components".to_string(),
        width: 600,
        height: 500,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, |ctx| {
        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::rgba(0.08, 0.08, 0.12, 1.0))
            .flex_col()
            .items_center()
            .p(32.0)
            .gap(16.0)
            .child(
                text("Reusable Components")
                    .size(28.0)
                    .weight(FontWeight::Bold)
                    .color(Color::WHITE),
            )
            .child(
                div()
                    .flex_col()
                    .gap(12.0)
                    .w(400.0)
                    .child(card(
                        "📱 Settings",
                        "Configure your preferences and app behavior",
                    ))
                    .child(card("ℹ️ About", "View app information and version"))
                    .child(card("❓ Help", "Get support and documentation"))
                    .child(card("🎨 Theme", "Customize the appearance")),
            )
    })
}

fn card(title: &str, description: &str) -> impl ElementBuilder {
    div()
        .class("card")
        .flex_row()
        .gap(16.0)
        .p(20.0)
        .bg(Color::rgba(0.15, 0.15, 0.2, 1.0))
        .rounded(12.0)
        .on_click(|_| {
            println!("Clicked: {}", title);
        })
        .child(
            div()
                .flex_center()
                .w(48.0)
                .h(48.0)
                .bg(Color::rgba(0.3, 0.4, 0.6, 1.0))
                .rounded(10.0)
                .child(
                    text(title.split_whitespace().next().unwrap())
                        .size(20.0)
                        .color(Color::WHITE),
                ),
        )
        .child(
            div()
                .flex_col()
                .gap(4.0)
                .flex_1()
                .child(
                    text(title)
                        .size(16.0)
                        .weight(FontWeight::SemiBold)
                        .color(Color::WHITE),
                )
                .child(
                    text(description)
                        .size(13.0)
                        .color(Color::rgba(0.6, 0.6, 0.6, 1.0)),
                ),
        )
}
