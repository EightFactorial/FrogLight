use alloc::boxed::Box;
use core::{
    fmt::Debug,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
    ops::{Deref, DerefMut},
};
use std::{io::ErrorKind, sync::OnceLock};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use hickory_resolver::{
    ResolveError, ResolveErrorKind,
    proto::{ProtoErrorKind, runtime::Executor},
};
use ureq::{
    Agent, Timeout,
    config::{Config, IpFamily},
    http::Uri,
    unversioned::{
        resolver::{ResolvedSocketAddrs, Resolver},
        transport::{DefaultConnector, NextTimeout},
    },
};

use super::DnsResolver;
use crate::resolver::RuntimeExecutor;

/// An HTTP client
///
/// Uses a shared [`ureq::Agent`] under the hood.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Resource, opaque))]
pub struct HttpClient(Agent);

impl Debug for HttpClient {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HttpClient").finish_non_exhaustive()
    }
}

impl Deref for HttpClient {
    type Target = Agent;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for HttpClient {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

static INSTANCE: OnceLock<HttpClient> = OnceLock::new();

impl HttpClient {
    /// Get the global [`HttpClient`].
    ///
    /// # Panics
    ///
    /// Panics if the global instance has not been initialized yet.
    #[must_use]
    pub fn get() -> &'static Self {
        INSTANCE.get().expect("The `HttpClient` has not been initialized yet!")
    }

    /// Attempts to get the global [`HttpClient`],
    /// or returns `None` if it is not initialized.
    #[must_use]
    pub fn try_get() -> Option<&'static Self> { INSTANCE.get() }

    /// Gets the global [`HttpClient`] instance, or initializes it with `f`.
    #[must_use]
    pub fn get_or_init(f: impl FnOnce() -> Agent) -> &'static Self {
        INSTANCE.get_or_init(|| Self(f()))
    }

    /// Gets the global [`HttpClient`] instance,
    /// or initializes it with the defaults.
    #[must_use]
    pub fn get_or_default() -> &'static Self {
        Self::get_or_init(|| {
            Agent::with_parts(
                Config::default(),
                DefaultConnector::default(),
                DnsResolver::get_or_default().clone(),
            )
        })
    }
}

// -------------------------------------------------------------------------------------------------

impl Resolver for DnsResolver {
    fn resolve(
        &self,
        uri: &Uri,
        config: &Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        /// Translate a `ResolveError` and convert it into a `ureq::Error`.
        fn translate(err: ResolveError) -> ureq::Error {
            match err.kind() {
                ResolveErrorKind::Proto(err_kind) => match err_kind.kind() {
                    ProtoErrorKind::NoRecordsFound { .. } => ureq::Error::HostNotFound,
                    ProtoErrorKind::NoConnections => ureq::Error::ConnectionFailed,
                    ProtoErrorKind::Timer | ProtoErrorKind::Timeout => {
                        ureq::Error::Timeout(Timeout::Resolve)
                    }
                    ProtoErrorKind::Io(err) if err.kind() == ErrorKind::TimedOut => {
                        ureq::Error::Timeout(Timeout::Resolve)
                    }
                    _ => ureq::Error::Other(Box::new(err)),
                },
                _ => ureq::Error::Other(Box::new(err)),
            }
        }

        if let Some(host) = uri.host() {
            let mut resolved = self.empty();
            let port = uri.port_u16().unwrap_or_else(|| match uri.scheme() {
                Some(scheme) if scheme.as_str() == "http" => 80,
                Some(scheme) if scheme.as_str() == "https" => 443,
                _ => 80,
            });

            // Perform the lookup based on the IP family configuration.
            match config.ip_family() {
                IpFamily::Any => match RuntimeExecutor.block_on(self.provider.lookup_ip(host)) {
                    Ok(result) => {
                        result.into_iter().for_each(|addr| {
                            resolved.push(SocketAddr::new(addr, port));
                        });
                    }
                    Err(err) => return Err(translate(err)),
                },
                IpFamily::Ipv4Only => {
                    match RuntimeExecutor.block_on(self.provider.ipv4_lookup(host)) {
                        Ok(result) => {
                            result.into_iter().for_each(|addr| {
                                resolved.push(SocketAddr::V4(SocketAddrV4::new(addr.0, port)));
                            });
                        }
                        Err(err) => return Err(translate(err)),
                    }
                }
                IpFamily::Ipv6Only => {
                    match RuntimeExecutor.block_on(self.provider.ipv6_lookup(host)) {
                        Ok(result) => {
                            result.into_iter().for_each(|addr| {
                                resolved
                                    .push(SocketAddr::V6(SocketAddrV6::new(addr.0, port, 0, 0)));
                            });
                        }
                        Err(err) => return Err(translate(err)),
                    }
                }
            }

            #[cfg(feature = "tracing")]
            tracing::debug!("Found addresses for \"{host}\": {resolved:?}");

            Ok(resolved)
        } else {
            Err(ureq::Error::HostNotFound)
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "bevy")]
impl bevy_ecs::world::FromWorld for HttpClient {
    fn from_world(world: &mut World) -> Self {
        world.get_resource::<Self>().unwrap_or_else(|| Self::get_or_default()).clone()
    }
}
