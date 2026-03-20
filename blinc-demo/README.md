# Blinc Examples

Working examples from `/home/glenn/projects/android-backup/blinc-demo/`.

## Setup

Add to `Cargo.toml`:

```toml
[dependencies]
blinc_app = { path = "/home/glenn/.cargo/git/checkouts/blinc-69f63bbdea5129d2/da94546/crates/blinc_app" }
blinc_animation = { path = "/home/glenn/.cargo/git/checkouts/blinc-69f63bbdea5129d2/da94546/crates/blinc_animation" }
blinc_layout = { path = "/home/glenn/.cargo/git/checkouts/blinc-69f63bbdea5129d2/da94546/crates/blinc_layout" }
blinc_core = { path = "/home/glenn/.cargo/git/checkouts/blinc-69f63bbdea5129d2/da94546/crates/blinc_core" }
```

## Run

```bash
cargo run --example 01_flexbox
cargo run --example 02_glass
cargo run --example 04_css_transitions
cargo run --example 10_complete_app
```

## Working Examples

| File | Feature |
|------|---------|
| 01_flexbox.rs | Flexbox layout |
| 02_glass.rs | Glass material |
| 04_css_transitions.rs | CSS hover effects |
| 10_complete_app.rs | Full app demo |

## Notes

- `flex_center()` does not exist - use `items_center()` + `justify_center()`
- State cloning pattern: `let c = count.clone();`
- Use `count.get()` and `count.set(val)` for state access
