#![doc = include_str!("../README.md")]
#![allow(clippy::alloc_instead_of_core, clippy::std_instead_of_core, reason = "Requires `std`")]
#![allow(unreachable_pub, reason = "Binary")]
#![allow(clippy::unnecessary_wraps, clippy::unused_async, reason = "WIP")]
#![allow(dead_code, unused_imports, reason = "WIP")]

use miette::{Result, bail};
use tokio::task::{JoinError, JoinSet};
use tracing_subscriber::EnvFilter;

use crate::source::{JarData, JarFile, Manifest};

mod common;
mod config;
mod generator;
mod helper;
mod source;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let mut tasks = JoinSet::<Result<()>>::new();

    // Load the configuration
    let config = config::load().await?;

    // Update crates' `Cargo.toml` files
    tasks.spawn(helper::CargoHelper::generate(config));
    // Generate `Version` structs
    tasks.spawn(helper::TypeHelper::generate(config));

    // Generate version-specific code (blocks, items, etc.)
    for version in &config.versions {
        tasks.spawn(generator::generate(version, config));
    }

    // Wait for all tasks to complete, returning the first error encountered.
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(err)) => return Err(err),
            Err(err) => bail!("Failed to join task, {err}"),
        }
    }

    Ok(())
}
