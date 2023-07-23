use std::{fs::File, io::Write};

use clap::Parser;
use cli::Commands;
use log::{error, info, warn, LevelFilter};
use mc_rs_ext::{
    extract::extract_data,
    print::print_data,
    search::search_data,
    types::{Manifest, Version},
};

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();
    setup_logger(cli.quiet);

    let manifest = match Manifest::get(cli.refresh) {
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

    if !cli.unstable && Version::new_release(1, 19, 4).is_newer(&version, &manifest) {
        error!("Only versions 1.19.4 and newer are supported!");
        warn!("Use -u or --unstable to allow using older versions.");
        return;
    } else if !cli.unstable && !version.is_stable() {
        error!("Version {} is not a stable release!", version);
        warn!("Use -u or --unstable to allow using unstable versions.");
        return;
    }

    match cli.command {
        Commands::Extract { datasets } => {
            match extract_data(&version, &manifest, datasets, cli.unstable) {
                Some(data) => output(json::stringify_pretty(data, 4), cli.output),
                None => error!("Failed to extract data!"),
            }
        }
        Commands::Search { query } => match search_data(&version, &manifest, query) {
            Some(data) => {
                if data.is_empty() {
                    info!("No results found!");
                } else {
                    output(data, cli.output);
                }
            }
            None => error!("Failed to search for query!"),
        },
        Commands::Print { class } => match print_data(&version, &manifest, class) {
            Some(data) => output(data, cli.output),
            None => error!("Failed to print data!"),
        },
    }
}

/// Setup logging for the application
fn setup_logger(quiet: bool) {
    let mut builder = env_logger::builder();

    if quiet {
        builder.filter_level(LevelFilter::Error);
    } else {
        #[cfg(debug_assertions)]
        {
            builder.filter_level(LevelFilter::Debug);
        }
        #[cfg(not(debug_assertions))]
        {
            builder.filter_level(LevelFilter::Info);
        }
    }

    builder.filter_module("reqwest", LevelFilter::Off);
    builder.format_timestamp(None);
    builder.init()
}

/// Print to console or write to file
fn output(data: String, output: Option<String>) {
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
