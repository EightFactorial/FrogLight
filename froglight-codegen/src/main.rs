#![doc = include_str!("../README.md")]
#![allow(clippy::std_instead_of_alloc, clippy::std_instead_of_core, reason = "Requires `std`")]
#![allow(dead_code, unreachable_pub, reason = "Binary")]

use miette::{Result, bail};
use tokio::task::JoinSet;
use tracing_subscriber::EnvFilter;

use crate::config::ConfigBundle;

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
    let config = ConfigBundle::load().await;

    // Generate `Cargo.toml` feature sets
    tasks.spawn(generator::cargo_toml::generate(config));
    // Generate `Version` structs
    tasks.spawn(generator::version_type::generate(config));

    // Generate `Version`-specific code (blocks, items, etc.)
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
