# Blinc UI Framework - Demo Guide

Examples demonstrating key Blinc features. Each section is self-contained and runnable.

## 1. Flexbox Layout

```rust
use blinc_app::prelude::*;

fn flexbox_demo() -> impl ElementBuilder {
    div()
        .flex_col()
        .gap(16.0)
        .p(24.0)
        .child(
            div()
                .flex_row()
                .gap(12.0)
                .child(text("Item 1").size(18.0))
                .child(text("Item 2").size(18.0))
                .child(text("Item 3").size(18.0))
        )
        .child(
            div()
                .flex_center()
                .w(200.0)
                .h(100.0)
                .bg(Color::rgba(0.2, 0.3, 0.5, 1.0))
                .rounded(12.0)
                .child(text("Centered").size(16.0))
        )
}
```

## 2. Glass Material

```rust
use blinc_app::prelude::*;

fn glass_demo() -> impl ElementBuilder {
    div()
        .w(400.0)
        .h(300.0)
        .flex_center()
        .bg(Color::rgba(0.1, 0.1, 0.2, 1.0))
        .child(
            div()
                .glass()
                .rounded(24.0)
                .p(32.0)
                .child(text("Frosted Glass").size(24.0).color(Color::WHITE))
        )
}
```

## 3. Spring Animations

```rust
use blinc_app::prelude::*;
use blinc_animation::SpringConfig;

fn animated_button() -> impl ElementBuilder {
    stateful::<ButtonState>()
        .on_state(|ctx| {
            let target = match ctx.state() {
                ButtonState::Hovered => 1.05,
                ButtonState::Pressed => 0.95,
                _ => 1.0,
            };
            let scale = ctx.use_spring("scale", target, SpringConfig::snappy());
            div()
                .bg(Color::rgba(0.3, 0.5, 0.9, 1.0))
                .rounded(12.0)
                .p(16.0)
                .scale(scale)
                .child(text("Click Me").size(18.0).color(Color::WHITE))
        })
}
```

## 4. CSS Transitions (No Rust Code)

```rust
use blinc_app::prelude::*;

fn main() -> Result<()> {
    let mut css_loaded = false;

    WindowedApp::run(config, move |ctx| {
        if !css_loaded {
            ctx.add_css(r#"
                .card {
                    background: #1e3a5f;
                    border-radius: 12px;
                    padding: 24px;
                    transition: all 300ms ease;
                }
                .card:hover {
                    background: #3b82f6;
                    transform: translateY(-4px);
                    box-shadow: 0 8px 20px rgba(59, 130, 246, 0.4);
                }
                .btn-primary {
                    background: #3b82f6;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: all 200ms ease;
                }
                .btn-primary:hover {
                    background: #2563eb;
                    transform: scale(1.02);
                }
                .btn-primary:active {
                    transform: scale(0.98);
                }
            "#);
            css_loaded = true;
        }

        div()
            .w(ctx.width)
            .h(ctx.height)
            .flex_col()
            .items_center()
            .justify_center()
            .gap(24.0)
            .child(div().class("card").child(text("Hover Me").size(20.0)))
            .child(div().class("btn-primary").child(text("Button").size(16.0)))
    })
}
```

## 5. Reactive State (Counter)

```rust
use blinc_app::prelude::*;

fn counter_demo(ctx: &WindowedContext) -> impl ElementBuilder {
    let count = ctx.use_state_keyed("counter", || 0i32);

    div()
        .flex_col()
        .items_center()
        .gap(16.0)
        .child(
            stateful::<NoState>()
                .deps([count.signal_id()])
                .on_state(move |_| {
                    let value = count.get();
                    text(&format!("Count: {}", value))
                        .size(48.0)
                        .weight(FontWeight::Bold)
                        .color(Color::WHITE)
                })
        )
        .child(
            div()
                .flex_row()
                .gap(12.0)
                .child(
                    div()
                        .bg(Color::rgba(0.6, 0.3, 0.3, 1.0))
                        .rounded(8.0)
                        .p(12.0)
                        .on_click(|_| { count.update(|v| v - 1); })
                        .child(text("-").size(24.0).color(Color::WHITE))
                )
                .child(
                    div()
                        .bg(Color::rgba(0.3, 0.6, 0.3, 1.0))
                        .rounded(8.0)
                        .p(12.0)
                        .on_click(|_| { count.update(|v| v + 1); })
                        .child(text("+").size(24.0).color(Color::WHITE))
                )
        )
}
```

## 6. Event Handling

```rust
use blinc_app::prelude::*;

fn event_demo() -> impl ElementBuilder {
    div()
        .flex_col()
        .gap(16.0)
        .child(
            div()
                .bg(Color::rgba(0.3, 0.3, 0.5, 1.0))
                .rounded(12.0)
                .p(24.0)
                .on_click(|ctx| {
                    println!("Clicked at: ({}, {})", ctx.local_x, ctx.local_y);
                })
                .on_hover_enter(|_| println!("Mouse entered"))
                .on_hover_leave(|_| println!("Mouse left"))
                .child(text("Click or hover me").size(16.0))
        )
        .child(
            div()
                .bg(Color::rgba(0.5, 0.3, 0.3, 1.0))
                .rounded(12.0)
                .p(24.0)
                .on_key_down(|ctx| {
                    if ctx.key_code == 13 {
                        println!("Enter pressed!");
                    }
                })
                .child(text("Press any key").size(16.0))
        )
}
```

## 7. Component Pattern

```rust
use blinc_app::prelude::*;

// Reusable card component
fn card(title: &str, content: &str) -> Div {
    div()
        .class("card")
        .flex_col()
        .gap(8.0)
        .p(20.0)
        .child(
            text(title)
                .size(20.0)
                .weight(FontWeight::SemiBold)
                .color(Color::WHITE)
        )
        .child(
            text(content)
                .size(14.0)
                .color(Color::rgba(0.7, 0.7, 0.7, 1.0))
        )
}

fn card_demo() -> impl ElementBuilder {
    div()
        .flex_col()
        .gap(16.0)
        .p(32.0)
        .child(card("Settings", "Configure your preferences"))
        .child(card("About", "Application information"))
        .child(card("Help", "Get support"))
}
```

## 8. Dark Theme with CSS Variables

```rust
use blinc_app::prelude::*;

fn dark_theme_demo(ctx: &WindowedContext) -> impl ElementBuilder {
    let mut css_loaded = false;

    WindowedApp::run(config, move |ctx| {
        if !css_loaded {
            ctx.add_css(r#"
                :root {
                    --bg-primary: #0f172a;
                    --bg-secondary: #1e293b;
                    --text-primary: #f8fafc;
                    --text-secondary: #94a3b8;
                    --accent: #3b82f6;
                }
                .theme-card {
                    background: var(--bg-secondary);
                    border-radius: 16px;
                    padding: 24px;
                    border: 1px solid var(--accent);
                }
                .theme-title {
                    color: var(--text-primary);
                    font-size: 24px;
                    font-weight: bold;
                }
                .theme-body {
                    color: var(--text-secondary);
                    font-size: 14px;
                }
                .theme-btn {
                    background: var(--accent);
                    border-radius: 8px;
                    padding: 12px 24px;
                    cursor: pointer;
                }
                .theme-btn:hover {
                    opacity: 0.9;
                }
            "#);
            css_loaded = true;
        }

        div()
            .w(ctx.width)
            .h(ctx.height)
            .bg(Color::from_hex("#0f172a").unwrap())
            .flex_col()
            .items_center()
            .justify_center()
            .gap(24.0)
            .child(
                div()
                    .class("theme-card")
                    .flex_col()
                    .gap(12.0)
                    .child(text("Dark Theme Demo").class("theme-title"))
                    .child(text("Using CSS variables").class("theme-body"))
                    .child(div().class("theme-btn").child(text("Click Me").size(14.0))))
            )
    })
}
```

## 9. Motion (Enter/Exit Animations)

```rust
use blinc_app::prelude::*;
use blinc_layout::motion::{motion, StaggerConfig, AnimationPreset};

fn motion_demo() -> impl ElementBuilder {
    div()
        .flex_col()
        .gap(16.0)
        .p(32.0)
        .child(
            motion()
                .fade_in(300)
                .slide_in(SlideDirection::Up, 200)
                .child(
                    div()
                        .bg(Color::rgba(0.2, 0.4, 0.6, 1.0))
                        .rounded(12.0)
                        .p(20.0)
                        .child(text("Animated Entry").size(18.0))
                )
        )
        .child(
            motion()
                .stagger(StaggerConfig::new(100, AnimationPreset::fade_in(300)))
                .child(
                    div()
                        .flex_col()
                        .gap(8.0)
                        .child(text("Item 1").size(14.0))
                        .child(text("Item 2").size(14.0))
                        .child(text("Item 3").size(14.0))
                )
        )
}
```

## 10. Complete App Example

```rust
use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};

fn main() -> Result<()> {
    let config = WindowConfig {
        title: "Blinc Demo".to_string(),
        width: 800,
        height: 600,
        resizable: true,
        ..Default::default()
    };

    WindowedApp::run(config, |ctx| build_app(ctx))
}

fn build_app(ctx: &WindowedContext) -> impl ElementBuilder {
    let count = ctx.use_state_keyed("demo_counter", || 0i32);

    div()
        .w(ctx.width)
        .h(ctx.height)
        .bg(Color::rgba(0.1, 0.1, 0.15, 1.0))
        .flex_col()
        .items_center()
        .justify_center()
        .gap(32.0)
        .child(
            div()
                .glass()
                .rounded(24.0)
                .p(48.0)
                .flex_col()
                .items_center()
                .gap(24.0)
                .child(text("Blinc Demo").size(32.0).weight(FontWeight::Bold).color(Color::WHITE))
                .child(
                    stateful::<NoState>()
                        .deps([count.signal_id()])
                        .on_state(move |_| {
                            text(&format!("Count: {}", count.get()))
                                .size(48.0)
                                .weight(FontWeight::Bold)
                                .color(Color::WHITE)
                        })
                )
                .child(
                    div()
                        .flex_row()
                        .gap(16.0)
                        .child(
                            div()
                                .bg(Color::rgba(0.6, 0.3, 0.3, 1.0))
                                .rounded(12.0)
                                .p(16.0)
                                .on_click(|_| { count.update(|v| v - 1); })
                                .child(text("-").size(28.0).weight(FontWeight::Bold).color(Color::WHITE))
                        )
                        .child(
                            div()
                                .bg(Color::rgba(0.3, 0.6, 0.3, 1.0))
                                .rounded(12.0)
                                .p(16.0)
                                .on_click(|_| { count.update(|v| v + 1); })
                                .child(text("+").size(28.0).weight(FontWeight::Bold).color(Color::WHITE))
                        )
                )
        )
}
```

## Key Takeaways

| Feature | Approach |
|---------|----------|
| Layout | Use `.flex_col()`, `.flex_row()`, `.gap()`, `.items_center()` |
| Styling | Prefer CSS with `ctx.add_css()` over builder methods |
| State | Use `ctx.use_state_keyed()` for reactive data |
| Animations | CSS transitions for hover effects, `ctx.use_spring()` for physics |
| Events | `.on_click()`, `.on_hover_enter()`, `.on_key_down()` |
| Theme | CSS variables via `:root { --var: value; }` |
