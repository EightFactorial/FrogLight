//! TODO

use bevy_log::tracing_subscriber::{EnvFilter, fmt};
use bevy_tasks::{IoTaskPool, TaskPool, block_on};
use froglight_resolver::prelude::*;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Initialize tracing and the `IoTaskPool`
    let _ = fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
    let _ = IoTaskPool::get_or_init(TaskPool::new);

    block_on(main_async())
}

async fn main_async() -> Result<(), Box<dyn core::error::Error>> {
    // Create a resolver using either the system configuration or Cloudflare DNS.
    let _resolver = FroglightResolver::system_config_or_cloudflare();

    Ok(())
}
