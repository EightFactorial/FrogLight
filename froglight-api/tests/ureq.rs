//! TODO

use async_io::block_on;
use froglight_api::{client::GetOptions, prelude::*};
use ureq::{Agent, config::Config, unversioned::transport::DefaultConnector};

fn client() -> HttpClient {
    #[cfg(feature = "bevy")]
    bevy_tasks::IoTaskPool::get_or_init(|| {
        bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
    });

    HttpClient::new(Agent::with_parts(
        Config::default(),
        DefaultConnector::default(),
        DnsResolver::default(),
    ))
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
    block_on(client.get("https://google.com", GetOptions {})).unwrap();
}

#[test]
fn minecraft_net() {
    #[cfg(feature = "tracing")]
    let _guard = trace();
    let client = client();
    block_on(client.get("https://minecraft.net", GetOptions {})).unwrap();
}
