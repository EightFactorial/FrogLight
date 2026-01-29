#![doc = include_str!("../README.md")]
#![allow(clippy::alloc_instead_of_core, clippy::std_instead_of_core, reason = "Requires `std`")]
#![allow(unreachable_pub, reason = "Binary")]
#![allow(clippy::unnecessary_wraps, clippy::unused_async, reason = "WIP")]
#![allow(dead_code, unused_imports, reason = "WIP")]

use miette::Result;
use tracing_subscriber::EnvFilter;

use crate::source::{JarData, JarFile, Manifest};

mod common;
use common::Version;

mod source;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let version = Version::new("1.21.11");
    JarData::get_for(&version, |_jar| async { Ok(()) }).await?;

    Ok(())
}
