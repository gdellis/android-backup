use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};
use blinc_layout::motion::{motion, AnimationPreset, StaggerConfig};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc - Motion Animations".to_string(),
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
            .gap(24.0)
            .child(
                text("Motion Animations")
                    .size(28.0)
                    .weight(FontWeight::Bold)
                    .color(Color::WHITE),
            )
            .child(
                div()
                    .flex_col()
                    .gap(16.0)
                    .w(400.0)
                    .child(
                        motion()
                            .fade_in(400)
                            .slide_in(SlideDirection::Up, 300)
                            .child(
                                div()
                                    .bg(Color::rgba(0.2, 0.3, 0.5, 1.0))
                                    .rounded(12.0)
                                    .p(20.0)
                                    .child(
                                        text("Fade In + Slide Up").size(16.0).color(Color::WHITE),
                                    ),
                            ),
                    )
                    .child(
                        motion()
                            .fade_in(300)
                            .scale(0.8, 1.0, SpringConfig::gentle())
                            .child(
                                div()
                                    .bg(Color::rgba(0.3, 0.2, 0.4, 1.0))
                                    .rounded(12.0)
                                    .p(20.0)
                                    .child(text("Fade In + Scale").size(16.0).color(Color::WHITE)),
                            ),
                    )
                    .child(
                        motion()
                            .stagger(StaggerConfig::new(100, AnimationPreset::fade_in(300)))
                            .child(
                                div()
                                    .flex_row()
                                    .gap(8.0)
                                    .child(
                                        div()
                                            .bg(Color::rgba(0.2, 0.5, 0.3, 1.0))
                                            .rounded(8.0)
                                            .p(16.0)
                                            .flex_center()
                                            .child(text("Item").size(14.0).color(Color::WHITE)),
                                    )
                                    .child(
                                        div()
                                            .bg(Color::rgba(0.5, 0.3, 0.2, 1.0))
                                            .rounded(8.0)
                                            .p(16.0)
                                            .flex_center()
                                            .child(text("Stagger").size(14.0).color(Color::WHITE)),
                                    )
                                    .child(
                                        div()
                                            .bg(Color::rgba(0.3, 0.4, 0.3, 1.0))
                                            .rounded(8.0)
                                            .p(16.0)
                                            .flex_center()
                                            .child(
                                                text("Animation").size(14.0).color(Color::WHITE),
                                            ),
                                    ),
                            ),
                    ),
            )
    })
}
