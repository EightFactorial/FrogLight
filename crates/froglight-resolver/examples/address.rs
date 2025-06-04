//! TODO

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use bevy_tasks::{IoTaskPool, TaskPool, block_on};
use froglight_resolver::prelude::*;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Initialize the tracing subscriber
    let filter = EnvFilter::try_from_default_env();
    let filter = filter.unwrap_or_else(|_| EnvFilter::new("address=info,frog=trace"));
    let _ = fmt().with_env_filter(filter).try_init();

    // Initialize the `IoTaskPool`
    let _ = IoTaskPool::get_or_init(TaskPool::new);

    block_on(main_async())
}

async fn main_async() -> Result<(), Box<dyn core::error::Error>> {
    // Create a resolver using Cloudflare's DNS servers
    let resolver = FroglightResolver::cloudflare();
    info!("Resolving addresses...");

    // Resolving addresses using the default port
    let addr = resolver.lookup_minecraft("127.0.0.1").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565));
    info!("127.0.0.1        -> {addr}\n");

    let addr = resolver.lookup_minecraft("::1").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 25565));
    info!("::1              -> {addr}\n");

    let addr = resolver.lookup_minecraft("localhost").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565));
    info!("localhost        -> {addr}\n");

    // Resolving addresses using a custom port
    let addr = resolver.lookup_minecraft("127.0.0.1:8080").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    info!("127.0.0.1:8080   -> {addr}\n");

    let addr = resolver.lookup_minecraft("[::1]:8080").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080));
    info!("[::1]:8080       -> {addr}\n");

    let addr = resolver.lookup_minecraft("localhost:8080").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    info!("localhost:8080   -> {addr}\n");

    // Resolving online addresses
    let addr = resolver.lookup_minecraft("hypixel.net").await?;
    assert_eq!(addr.port(), 25565);
    info!("hypixel.net      -> {addr}\n");

    let addr = resolver.lookup_minecraft("mc.hypixel.net").await?;
    assert_eq!(addr.port(), 25565);
    info!("mc.hypixel.net   -> {addr}\n");

    let addr = resolver.lookup_minecraft("hypixel.net:8080").await?;
    assert_eq!(addr.port(), 8080);
    info!("hypixel.net:8080 -> {addr}\n");

    Ok(())
}
