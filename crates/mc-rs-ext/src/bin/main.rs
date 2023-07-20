use clap::Parser;
use cli::Commands;
use log::{info, LevelFilter};
use mc_rs_ext::types::Manifest;

use crate::cli::Cli;

mod cli;

fn main() -> anyhow::Result<()> {
    setup_logger();
    let cli = Cli::parse();

    let (refresh, unstable) = match cli.command {
        Commands::Extract {
            refresh, unstable, ..
        } => (refresh, unstable),
        _ => (false, false),
    };

    let manifest = Manifest::get_manifest(refresh)?;

    let version = cli
        .command
        .version()
        .clone()
        .unwrap_or_else(|| manifest.get_latest(unstable));

    if !unstable && !version.is_stable() {
        return Err(anyhow::Error::msg(
            "The selected version is not a stable release! Use -u or --unstable to extract it anyway!",
        ));
    }

    info!("Selected version: {}", version);

    Ok(())
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
