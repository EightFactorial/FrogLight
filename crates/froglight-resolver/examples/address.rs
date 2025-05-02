//! TODO

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use bevy_tasks::{IoTaskPool, TaskPool, block_on};
use froglight_resolver::prelude::*;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Initialize the `IoTaskPool`
    let _ = IoTaskPool::get_or_init(TaskPool::new);

    block_on(main_async())
}

async fn main_async() -> Result<(), Box<dyn core::error::Error>> {
    // Create a resolver using Cloudflare's DNS servers
    let resolver = FroglightResolver::cloudflare();
    println!("Resolving addresses...");

    // Resolving addresses using the default port
    let addr = resolver.lookup_minecraft("127.0.0.1").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565));
    println!("127.0.0.1        -> {addr}");

    let addr = resolver.lookup_minecraft("::1").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 25565));
    println!("::1              -> {addr}");

    let addr = resolver.lookup_minecraft("localhost").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565));
    println!("localhost        -> {addr}");

    println!();

    // Resolving addresses using a custom port
    let addr = resolver.lookup_minecraft("127.0.0.1:8080").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    println!("127.0.0.1:8080   -> {addr}");

    let addr = resolver.lookup_minecraft("[::1]:8080").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080));
    println!("[::1]:8080       -> {addr}");

    let addr = resolver.lookup_minecraft("localhost:8080").await?;
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    println!("localhost:8080   -> {addr}");

    println!();

    // Resolving online addresses
    let addr = resolver.lookup_minecraft("hypixel.net").await?;
    assert_eq!(addr.port(), 25565);
    println!("hypixel.net      -> {addr}");

    let addr = resolver.lookup_minecraft("mc.hypixel.net").await?;
    assert_eq!(addr.port(), 25565);
    println!("mc.hypixel.net   -> {addr}");

    let addr = resolver.lookup_minecraft("hypixel.net:8080").await?;
    assert_eq!(addr.port(), 8080);
    println!("hypixel.net:8080 -> {addr}");

    Ok(())
}
