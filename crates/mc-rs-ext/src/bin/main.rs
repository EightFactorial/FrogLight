use clap::Parser;
use cli::Commands;
use log::{error, info, warn, LevelFilter};
use mc_rs_ext::types::Manifest;

use crate::cli::Cli;

mod cli;

fn main() {
    setup_logger();
    let cli = Cli::parse();

    let (refresh, unstable) = match cli.command {
        Commands::Extract {
            refresh, unstable, ..
        } => (refresh, unstable),
        _ => (false, false),
    };

    let manifest = match Manifest::get_manifest(refresh) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to get manifest: {}", err);
            return;
        }
    };

    let version = cli
        .command
        .version()
        .clone()
        .unwrap_or_else(|| manifest.get_latest(unstable));

    if !manifest.versions.iter().any(|v| v.id == version) {
        error!("Version {} not found in the version manifest!", version);
        warn!("Use -r or --refresh to redownload the version manifest if the version was recently released.");
        return;
    }

    if !unstable && !version.is_stable() {
        error!("Version {} is not stable!", version);
        warn!("Use -u or --unstable to allow using unstable versions.");
        return;
    }

    info!("Selected version: {}", version);
}

/// Setup logging for the application
fn setup_logger() {
    let mut builder = env_logger::builder();

    #[cfg(debug_assertions)]
    {
        builder.filter_level(LevelFilter::Debug);
    }
    #[cfg(not(debug_assertions))]
    {
        builder.filter_level(LevelFilter::Info);
    }

    builder.filter_module("reqwest", LevelFilter::Off);
    builder.format_timestamp(None);
    builder.init()
}
