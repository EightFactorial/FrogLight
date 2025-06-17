//! A [`ureq::Agent`], backed by a [`FroglightResolver`]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::Deref;
use hickory_resolver::{ResolveErrorKind, proto::ProtoErrorKind};
use ureq::{
    Agent,
    config::Config,
    http::{Uri, uri::Scheme},
    unversioned::{
        resolver::{ResolvedSocketAddrs, Resolver},
        transport::{Connector, DefaultConnector, NextTimeout},
    },
};

use crate::resolver::FroglightResolver;

/// A thread-safe wrapper around an [`Agent`] for use in Bevy.
///
/// Uses [`FroglightResolver`] for DNS resolution if enabled.
#[derive(Debug, Clone, Deref)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, Resource))]
pub struct FroglightAgent(Agent);

impl FroglightAgent {
    /// Create a new [`FroglightAgent`] using the given [`FroglightResolver`].
    #[must_use]
    pub fn new(resolver: &FroglightResolver) -> Self {
        Self::from_parts(resolver.clone(), Config::default(), DefaultConnector::new())
    }

    /// Create a new [`FroglightAgent`] using the given parts.
    #[must_use]
    pub fn from_parts(
        resolver: FroglightResolver,
        config: Config,
        connector: impl Connector,
    ) -> Self {
        Self(Agent::with_parts(config, connector, resolver))
    }
}

#[cfg(feature = "bevy")]
impl bevy_ecs::world::FromWorld for FroglightAgent {
    #[allow(unused_variables)]
    fn from_world(world: &mut World) -> Self { Self::new(&world.get_resource_or_init()) }
}

// -------------------------------------------------------------------------------------------------

impl Resolver for FroglightResolver {
    fn resolve(
        &self,
        uri: &Uri,
        _config: &Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        let host = uri.host().ok_or_else(|| ureq::Error::BadUri(uri.to_string()))?;
        let scheme = uri.scheme().unwrap_or(&Scheme::HTTP);

        let port = uri.port_u16().unwrap_or_else(|| match scheme.as_str() {
            "https" => 443,
            "socks" | "socks4" | "socks4a" | "socks5" => 1080,
            _ => 80,
        });

        match futures_lite::future::block_on(self.lookup_ip(host)) {
            Ok(addrs) => {
                let mut result = ResolvedSocketAddrs::from_fn(|_| {
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
                });
                for addr in addrs.into_iter().take(16) {
                    result.push(SocketAddr::new(addr, port));
                }
                Ok(result)
            }
            Err(err) => match err.kind() {
                ResolveErrorKind::Proto(err) if matches!(err.kind(), ProtoErrorKind::Timeout) => {
                    Err(ureq::Error::Timeout(ureq::Timeout::Resolve))
                }
                _ => Err(ureq::Error::Io(std::io::Error::from(err))),
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(test)]
fn agent() {
    use std::io::ErrorKind;

    use tracing_subscriber::{EnvFilter, fmt};

    // Initialize the tracing subscriber
    if let Ok(filter) = EnvFilter::try_from_default_env() {
        let _ = fmt().with_env_filter(filter).try_init();
    }

    // Initialize the `IoTaskPool` if the `bevy` feature is enabled
    #[cfg(feature = "bevy")]
    bevy_tasks::IoTaskPool::get_or_init(bevy_tasks::TaskPool::new);

    // Create a FroglightAgent using Cloudflare's DNS
    let agent = FroglightAgent::new(&FroglightResolver::cloudflare());

    // Attempt to connect to ip addresses
    match agent.get("http://127.0.0.1").call() {
        Ok(..) | Err(ureq::Error::ConnectionFailed) => {}
        Err(ureq::Error::Io(err)) if err.kind() == ErrorKind::ConnectionRefused => {}
        Err(err) => panic!("Failed to connect to \"127.0.0.1\": {err}"),
    }
    match agent.get("https://1.1.1.1").call() {
        Ok(..) | Err(ureq::Error::ConnectionFailed) => {}
        Err(ureq::Error::Io(err)) if err.kind() == ErrorKind::ConnectionRefused => {}
        Err(err) => panic!("Failed to connect to \"1.1.1.1\": {err}"),
    }

    // Attempt to resolve and connect to web addresses
    match agent.get("https://www.google.com").call() {
        Ok(..) | Err(ureq::Error::ConnectionFailed) => {}
        Err(ureq::Error::Io(err)) if err.kind() == ErrorKind::ConnectionRefused => {}
        Err(err) => panic!("Failed to connect to \"https://www.google.com\": {err}"),
    }
}
