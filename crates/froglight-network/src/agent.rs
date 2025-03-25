//! [`FroglightAgent`] for use in Bevy.

use std::sync::Arc;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "resolver")]
use hickory_resolver::{ResolveErrorKind, proto::ProtoErrorKind};
use ureq::Agent;

#[cfg(feature = "resolver")]
use crate::resolver::FroglightResolver;

/// A thread-safe wrapper around an [`Agent`] for use in Bevy.
///
/// Uses [`FroglightResolver`] for DNS resolution if enabled.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect), reflect(opaque, Debug, Resource))]
pub struct FroglightAgent(Arc<Agent>);

impl FroglightAgent {
    /// Create a new [`FroglightAgent`] using the given [`FroglightResolver`].
    #[must_use]
    #[cfg(feature = "resolver")]
    pub fn new(resolver: &FroglightResolver) -> Self {
        Self(Arc::new(Agent::with_parts(
            ureq::config::Config::default(),
            ureq::unversioned::transport::DefaultConnector::new(),
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

#[cfg(feature = "bevy")]
impl bevy_ecs::world::FromWorld for FroglightAgent {
    #[allow(unused_variables)]
    fn from_world(world: &mut World) -> Self {
        #[cfg(feature = "resolver")]
        {
            Self::new(&world.get_resource_or_init())
        }

        #[cfg(not(feature = "resolver"))]
        {
            Self::new_without_resolver()
        }
    }
}

#[cfg(feature = "resolver")]
impl ureq::unversioned::resolver::Resolver for FroglightResolver {
    fn resolve(
        &self,
        uri: &ureq::http::Uri,
        _config: &ureq::config::Config,
        _timeout: ureq::unversioned::transport::NextTimeout,
    ) -> Result<ureq::unversioned::resolver::ResolvedSocketAddrs, ureq::Error> {
        let host = uri.host().ok_or_else(|| ureq::Error::BadUri(uri.to_string()))?;
        let scheme = uri.scheme().unwrap_or(&ureq::http::uri::Scheme::HTTP);

        let port = uri.port_u16().unwrap_or_else(|| match scheme.as_str() {
            "https" => 443,
            "socks" | "socks4" | "socks4a" | "socks5" => 1080,
            _ => 80,
        });

        match futures_lite::future::block_on(self.lookup_ip(host)) {
            Ok(addrs) => {
                let mut result = ureq::unversioned::resolver::ResolvedSocketAddrs::from_fn(|_| {
                    std::net::SocketAddr::new(
                        std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
                        0,
                    )
                });
                for addr in addrs.into_iter().take(16) {
                    result.push(std::net::SocketAddr::new(addr, port));
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

    #[cfg(feature = "resolver")]
    let agent = {
        use hickory_resolver::config::{ResolverConfig, ResolverOpts};

        #[cfg(feature = "bevy")]
        bevy_tasks::IoTaskPool::get_or_init(bevy_tasks::TaskPool::new);

        FroglightAgent::new(&FroglightResolver::new(
            ResolverConfig::cloudflare(),
            ResolverOpts::default(),
        ))
    };

    #[cfg(not(feature = "resolver"))]
    let agent = FroglightAgent::new_without_resolver();

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
