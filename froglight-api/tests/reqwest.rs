//! TODO

use async_compat::Compat;
use async_io::block_on;
use froglight_api::{client::GetOptions, prelude::*};
use reqwest::Client;

fn client() -> HttpClient {
    #[cfg(feature = "bevy")]
    bevy_tasks::IoTaskPool::get_or_init(bevy_tasks::TaskPool::new);

    HttpClient::new(Client::builder().dns_resolver(DnsResolver::default()).build().unwrap())
}

#[cfg(feature = "tracing")]
fn trace() -> tracing::subscriber::DefaultGuard {
    use tracing_subscriber::prelude::*;
    let subscriber =
        tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().with_test_writer());
    tracing::subscriber::set_default(subscriber)
}

// -------------------------------------------------------------------------------------------------

#[test]
fn google_com() {
    #[cfg(feature = "tracing")]
    let _guard = trace();
    let client = client();
    block_on(Compat::new(client.get("https://google.com", GetOptions {}))).unwrap();
}

#[test]
fn minecraft_net() {
    #[cfg(feature = "tracing")]
    let _guard = trace();
    let client = client();
    block_on(Compat::new(client.get("https://minecraft.net", GetOptions {}))).unwrap();
}
