//! TODO

use async_io::block_on;
use froglight_api::{
    api::{ClientApi, Mojang},
    prelude::*,
};
use ureq::{Agent, config::Config, unversioned::transport::DefaultConnector};
use uuid::Uuid;

fn api() -> (ClientApi, HttpClient) {
    #[cfg(feature = "bevy")]
    bevy_tasks::IoTaskPool::get_or_init(|| {
        bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
    });

    (
        ClientApi::new(Mojang),
        HttpClient::new(Agent::with_parts(
            Config::default(),
            DefaultConnector::default(),
            DnsResolver::default(),
        )),
    )
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
fn player_uuid() {
    const INPUT: &str = "Mr_Sus_";
    const EXPECTED: Uuid = Uuid::from_u128_le(259778710492803530310996621428516138805);

    #[cfg(feature = "tracing")]
    let _guard = trace();
    let (api, client) = api();
    let uuid = block_on(api.query_uuid(INPUT, &client)).unwrap();
    assert_eq!(uuid, Some(EXPECTED), "Got \"{uuid:?}\", expected \"Some({EXPECTED})\"");
}

#[test]
fn player_username() {
    const INPUT: Uuid = Uuid::from_u128_le(259778710492803530310996621428516138805);
    const EXPECTED: &str = "Mr_Sus_";

    #[cfg(feature = "tracing")]
    let _guard = trace();
    let (api, client) = api();
    let username = block_on(api.query_username(INPUT, &client)).unwrap();
    assert_eq!(
        username.as_deref(),
        Some(EXPECTED),
        "Got \"{username:?}\", expected \"Some({EXPECTED})\""
    );
}
