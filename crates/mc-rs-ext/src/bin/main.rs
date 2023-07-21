use std::{fs::File, io::Write};

use clap::Parser;
use cli::Commands;
use log::{error, info, warn, LevelFilter};
use mc_rs_ext::{extract::extract_data, print::print_data, types::Manifest};

use crate::cli::Cli;

mod cli;

fn main() {
    setup_logger();

    let cli = Cli::parse();

    let manifest = match Manifest::get_manifest(cli.refresh) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to get manifest: {}", err);
            return;
        }
    };

    let version = cli
        .version
        .unwrap_or_else(|| manifest.get_latest(cli.unstable));

    if !manifest.versions.iter().any(|v| v.id == version) {
        error!("Version {} not found in the manifest!", version);
        warn!("Use -r or --refresh to redownload the manifest if it was recently released.");
        return;
    }

    if !cli.unstable && !version.is_stable() {
        error!("Version {} is not a stable release!", version);
        warn!("Use -u or --unstable to allow using unstable versions.");
        return;
    }

    info!("Selected version: {}", version);

    match cli.command {
        Commands::Extract { datasets } => match extract_data(version, manifest, datasets) {
            Some(data) => output_data(json::stringify_pretty(data, 4), cli.output),
            None => error!("Failed to extract data!"),
        },
        Commands::Search { .. } => todo!(),
        Commands::Print { class } => match print_data(version, manifest, class) {
            Some(data) => output_data(data, cli.output),
            None => error!("Failed to print data!"),
        },
    }
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

/// Print to console or write to file
fn output_data(data: String, output: Option<String>) {
    if let Some(path) = output {
        info!("Writing to {}", path);
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(err) => {
                error!("Failed to create file: {}", err);
                return;
            }
        };

        file.write_all(data.as_bytes()).unwrap();
    } else {
        info!("Writing to console");
        println!("{}", data);
    }
}
