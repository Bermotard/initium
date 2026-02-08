// Initium - Dashboard Intelligent
// Entry point de l'application

use log::{error, info};
use std::env;

mod config;
mod launcher;
mod system;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("🚀 Starting Initium v0.1.0-alpha");

    // TODO: Implémenter application
    // 1. Load config
    // 2. Setup UI
    // 3. Register autostart
    // 4. Run event loop

    info!("✅ Initium ready");
    Ok(())
}
