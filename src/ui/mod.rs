#[cfg(feature = "blinc-ui")]
mod main_window;

#[cfg(feature = "blinc-ui")]
pub use main_window::run_ui;

#[cfg(not(feature = "blinc-ui"))]
pub fn run_ui() -> Result<()> {
    anyhow::bail!("UI support not enabled. Build with --features blinc-ui")
}
