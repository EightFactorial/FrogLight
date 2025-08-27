//! TODO

use froglight_network::prelude::DnsResolver;
use futures_lite::future::block_on;

#[cfg(feature = "tracing")]
fn init_subscriber() {
    use tracing_subscriber::{EnvFilter, fmt};
    let env =
        EnvFilter::builder().parse_lossy("hickory_proto=error,rustls=error,ureq_proto=error,debug");
    let _ = fmt().with_env_filter(env).try_init();
}

#[test]
fn resolve_ip() {
    const HOSTNAME: &str = "google.com";

    #[cfg(feature = "tracing")]
    init_subscriber();

    match block_on(DnsResolver::get_or_default().lookup_ip(HOSTNAME)) {
        Ok(result) => println!("Found IP \"{HOSTNAME}\": {result:?}"),
        Err(err) => panic!("Failed to lookup \"{HOSTNAME}\": {err}"),
    }
}

#[test]
fn resolve_txt() {
    const HOSTNAME: &str = "google.com";

    #[cfg(feature = "tracing")]
    init_subscriber();

    match block_on(DnsResolver::get_or_default().lookup_txt(HOSTNAME)) {
        Ok(result) => println!("Found TXT \"{HOSTNAME}\": {result:?}"),
        #[cfg(feature = "tracing")]
        Err(err) => tracing::error!("Failed to lookup \"{HOSTNAME}\": {err}"),
        #[cfg(not(feature = "tracing"))]
        Err(err) => println!("Failed to lookup \"{HOSTNAME}\": {err}"),
    }
}

#[test]
fn resolve_srv() {
    const HOSTNAME: &str = "_minecraft._tcp.hypixel.net";

    #[cfg(feature = "tracing")]
    init_subscriber();

    match block_on(DnsResolver::get_or_default().lookup_srv(HOSTNAME)) {
        Ok(result) => println!("Found SRV \"{HOSTNAME}\": {result:?}"),
        Err(err) => panic!("Failed to lookup \"{HOSTNAME}\": {err}"),
    }
}

#[test]
fn resolve_minecraft() {
    const HOSTNAME: &str = "hypixel.net";

    #[cfg(feature = "tracing")]
    init_subscriber();

    match block_on(DnsResolver::get_or_default().lookup_minecraft(HOSTNAME)) {
        Ok(result) => println!("Found Minecraft Server \"{HOSTNAME}\": {result}"),
        Err(err) => panic!("Failed to lookup \"{HOSTNAME}\": {err}"),
    }
}
