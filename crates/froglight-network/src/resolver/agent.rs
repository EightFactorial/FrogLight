use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use hickory_resolver::error::ResolveErrorKind;
use ureq::{
    Agent, Timeout,
    config::Config,
    http::{Uri, uri::Scheme},
    unversioned::{
        resolver::{ResolvedSocketAddrs, Resolver},
        transport::{DefaultConnector, NextTimeout},
    },
};

use super::FroglightResolver;

/// A thread-safe wrapper around an [`Agent`] for use in Bevy.
///
/// Used for making HTTP/HTTPS requests
/// using a [`FroglightResolver`] for DNS resolution.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect), reflect(opaque, Debug, Resource))]
pub struct FroglightAgent(Arc<Agent>);

impl FroglightAgent {
    /// Create a new [`FroglightAgent`] using the given [`FroglightResolver`].
    #[must_use]
    pub fn new(resolver: &FroglightResolver) -> Self {
        Self(Arc::new(Agent::with_parts(
            Config::default(),
            DefaultConnector::new(),
            resolver.clone(),
        )))
    }

    /// Create a new [`FroglightAgent`] with default settings.
    ///
    /// See [`Agent::new_with_defaults`] for more information.
    #[must_use]
    pub fn new_without_resolver() -> Self { Self(Arc::new(Agent::new_with_defaults())) }
}

impl std::ops::Deref for FroglightAgent {
    type Target = Agent;

    fn deref(&self) -> &Self::Target { &self.0 }
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
                ResolveErrorKind::Timeout => Err(ureq::Error::Timeout(Timeout::Resolve)),
                _ => Err(ureq::Error::Io(std::io::Error::from(err))),
            },
        }
    }
}

#[test]
#[cfg(test)]
fn agent() {
    use std::io::ErrorKind;

    #[cfg(feature = "bevy")]
    let agent = {
        use hickory_resolver::config::{ResolverConfig, ResolverOpts};

        bevy_tasks::IoTaskPool::get_or_init(bevy_tasks::TaskPool::new);
        let resolver =
            FroglightResolver::new(ResolverConfig::cloudflare(), ResolverOpts::default());

        FroglightAgent::new(&resolver)
    };

    #[cfg(not(feature = "bevy"))]
    let agent = FroglightAgent::new_with_defaults();

    // Attempt to connect to local addresses
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

    // Attempt to resolve and connect to global addresses
    match agent.get("https://www.google.com").call() {
        Ok(..) | Err(ureq::Error::ConnectionFailed) => {}
        Err(ureq::Error::Io(err)) if err.kind() == ErrorKind::ConnectionRefused => {}
        Err(err) => panic!("Failed to connect to \"https://www.google.com\": {err}"),
    }
    match agent.get("https://github.com").call() {
        Ok(..) | Err(ureq::Error::ConnectionFailed) => {}
        Err(ureq::Error::Io(err)) if err.kind() == ErrorKind::ConnectionRefused => {}
        Err(err) => panic!("Failed to connect to \"https://github.com\": {err}"),
    }
}
