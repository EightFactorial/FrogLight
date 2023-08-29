use clap::Parser;
use cli::Cli;
use generate::{Generator, Generators};
use git2::Repository;
use itertools::Itertools;
use log::{error, info, warn, LevelFilter};
use mc_rs_ext::{
    extract::extract_data,
    types::{Manifest, Version},
};
use strum::IntoEnumIterator;

use crate::generate::format::Format;

mod cli;
mod generate;
mod util;

fn main() {
    let cli = Cli::parse();
    setup_logger();

    // Get the version manifest
    let manifest = match Manifest::get(cli.refresh) {
        Ok(manifest) => manifest,
        Err(err) => {
            error!("Failed to get manifest: {}", err);
            return;
        }
    };

    if !manifest.versions.iter().any(|v| v.id == cli.version) {
        error!("Version {} not found in the manifest!", cli.version);
        warn!("Use -r or --refresh to redownload the manifest if it was recently released.");
        return;
    }

    if !cli.unstable && Version::new_release(1, 19, 4).is_newer(&cli.version, &manifest) {
        error!("Only versions 1.19.4 and newer are supported!");
        warn!("Use -u or --unstable to allow using older versions.");
        return;
    } else if !cli.unstable && !cli.version.is_stable() {
        error!("Version {} is not a stable release!", cli.version);
        warn!("Use -u or --unstable to allow using unstable versions.");
        return;
    }

    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(err) => {
            error!("Failed to find git repository: {}", err);
            return;
        }
    };

    // Get the generators to run
    let mut generators = cli
        .generators
        .unwrap_or_else(|| Generators::iter().collect_vec());

    // Add `Format` if not already present
    if !generators.contains(&Generators::Format(Format)) {
        generators.push(Generators::Format(Format));
    }

    // Find all required datasets to use the selected generators
    let mut required = Vec::new();
    for gen in generators.iter() {
        required.extend_from_slice(gen.deps());
    }
    required = required.into_iter().unique().collect_vec();

    // Extract the required data
    let data = match extract_data(&cli.version, &manifest, Some(required), cli.unstable) {
        Some(data) => data,
        None => {
            error!("Failed to extract data for {}", cli.version);
            return;
        }
    };

    // Run the generators
    info!("");
    for gen in generators.iter() {
        info!("Generating {:?}...", gen);
        gen.parse(&cli.version, &data, &repo);
    }

    info!("");
    info!("Generation complete!");

    warn!("");
    warn!("-- Remember to check all generated code! --");
    warn!("");
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
