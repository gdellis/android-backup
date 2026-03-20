# Blinc Examples

Run individual examples with:

```bash
# Add blinc to a separate project
cargo new --name blinc-demo
cd blinc-demo

# Add to Cargo.toml:
[dependencies]
blinc_app = { git = "https://github.com/project-blinc/Blinc" }
blinc_animation = { git = "https://github.com/project-blinc/Blinc" }
blinc_layout = { git = "https://github.com/project-blinc/Blinc" }
blinc_core = { git = "https://github.com/project-blinc/Blinc" }

# Copy example code to src/main.rs
# Then run:
cargo run
```

Or use cargo's example feature:

```bash
# Create examples/ folder in your blinc project
# Copy example files there
cargo run --example 01_flexbox
```

## Examples

| File | Feature |
|------|---------|
| 01_flexbox.rs | Flexbox layout with columns, rows, gap |
| 02_glass.rs | Glass material effect |
| 03_springs.rs | Spring animations on buttons |
| 04_css_transitions.rs | CSS-driven hover effects |
| 05_counter.rs | Reactive state with signals |
| 06_events.rs | Click, hover, key events |
| 07_components.rs | Reusable card components |
| 08_dark_theme.rs | CSS variables theming |
| 09_motion.rs | Motion/enter animations |
| 10_complete_app.rs | Full app combining all features |
