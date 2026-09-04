//! RustyPixel dedicated server.
//!
//! Phase 0: placeholder. Loads `server.toml`, initializes logging, prints a
//! startup banner and exits gracefully. No real networking until Phase 8.

use std::path::Path;
use std::time::Duration;

use rustypixel_engine::config::ServerConfig;

fn main() {
    tracing_subscriber::fmt::init();

    // Load server.toml from the current directory; fall back to defaults
    // when the file is missing or unreadable.
    let config = ServerConfig::load(Path::new("server.toml"))
        .ok()
        .flatten()
        .unwrap_or_default();

    tracing::info!("RustyPixel server starting...");
    tracing::info!(
        name = %config.name,
        max_players = config.max_players,
        port = config.port,
        "server config loaded"
    );

    // Placeholder: a short idle loop, then exit gracefully. Real networking
    // replaces this in Phase 8.
    std::thread::sleep(Duration::from_millis(100));
    tracing::info!("RustyPixel server shutting down (placeholder).");
}