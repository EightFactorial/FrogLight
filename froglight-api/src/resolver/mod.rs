//! TODO

use std::{
    error::Error,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

use async_trait::async_trait;
#[cfg(feature = "bevy")]
use bevy_ecs::{reflect::ReflectResource, resource::Resource};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
#[cfg(feature = "reqwest")]
use reqwest::dns::{Name, Resolve as ReqwestResolve, Resolving};
#[cfg(feature = "ureq")]
use ureq::{
    config::Config,
    http::Uri,
    unversioned::{
        resolver::{ResolvedSocketAddrs, Resolver as UreqResolver},
        transport::NextTimeout,
    },
};

#[cfg(feature = "resolver")]
pub mod hickory;
#[cfg(feature = "resolver")]
pub use hickory::Resolver;

/// A DNS resolver for performing record lookups.
///
/// Supports multiple underlying implementations via the [`NetworkResolver`]
/// trait.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, Default, Resource))]
pub struct DnsResolver(Arc<dyn NetworkResolver>);

impl DnsResolver {
    /// Creates a new [`Resolver`] from a [`NetworkResolver`].
    #[inline]
    #[must_use]
    pub fn new<T: NetworkResolver>(agent: T) -> Self { Self::new_arc(Arc::new(agent)) }

    /// Creates a new [`Resolver`] from an [`Arc<dyn NetworkResolver>`].
    #[inline]
    #[must_use]
    pub const fn new_arc(agent: Arc<dyn NetworkResolver>) -> Self { Self(agent) }

    /// Returns a reference to the inner [`Arc<dyn NetworkResolver>`].
    #[inline]
    #[must_use]
    pub const fn as_arc(&self) -> &Arc<dyn NetworkResolver> { &self.0 }

    /// Spawns a task on bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool) to
    /// run the given async function with this resolver.
    #[inline]
    #[cfg(feature = "bevy")]
    pub fn spawn_task_using<F: AsyncFnOnce(Self) + 'static>(&self, f: F) {
        bevy_tasks::IoTaskPool::get().spawn(f(self.clone())).detach();
    }

    /// Resolves the given name to an iterator of [`IpAddr`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    pub async fn lookup_ip(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = IpAddr> + Send>, Box<dyn Error + Send + Sync>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::resolver", "Resolving IP for \"{name}\"");
        self.0.lookup_ip(name).await
    }

    /// Resolves the given name to an iterator of [`Ipv4Addr`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    pub async fn lookup_ipv4(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv4Addr> + Send>, Box<dyn Error + Send + Sync>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::resolver", "Resolving IPv4 for \"{name}\"");
        self.0.lookup_ipv4(name).await
    }

    /// Resolves the given name to an iterator of [`Ipv6Addr`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    pub async fn lookup_ipv6(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv6Addr> + Send>, Box<dyn Error + Send + Sync>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::resolver", "Resolving IPv6 for \"{name}\"");
        self.0.lookup_ipv6(name).await
    }

    /// Resolves the given name to an iterator of nameserver [`String`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    pub async fn lookup_ns(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = String> + Send>, Box<dyn Error + Send + Sync>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::resolver", "Resolving NS for \"{name}\"");
        self.0.lookup_ns(name).await
    }

    /// Resolves the given name to an iterator of SRV records as
    /// `(target, port)` tuples.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    pub async fn lookup_srv(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = (String, u16)> + Send>, Box<dyn Error + Send + Sync>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_api::resolver", "Resolving SRV for \"{name}\"");
        self.0.lookup_srv(name).await
    }
}

impl Debug for DnsResolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DnsResolver").field(&"Arc<dyn NetworkResolver>").finish()
    }
}

#[cfg(feature = "resolver")]
impl Default for DnsResolver {
    #[inline]
    fn default() -> Self { Self::new(Resolver::default()) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can act as network agents.
#[async_trait]
pub trait NetworkResolver: Send + Sync + 'static {
    /// Resolves the given name to an iterator of [`IpAddr`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    async fn lookup_ip(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = IpAddr> + Send>, Box<dyn Error + Send + Sync>>;

    /// Resolves the given name to an iterator of [`Ipv4Addr`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    async fn lookup_ipv4(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv4Addr> + Send>, Box<dyn Error + Send + Sync>>;

    /// Resolves the given name to an iterator of [`Ipv6Addr`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    async fn lookup_ipv6(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv6Addr> + Send>, Box<dyn Error + Send + Sync>>;

    /// Resolves the given name to an iterator of nameserver [`String`]s.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    async fn lookup_ns(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = String> + Send>, Box<dyn Error + Send + Sync>>;

    /// Resolves the given name to an iterator of SRV records as `(target,
    /// port)` tuples.
    ///
    /// # Errors
    ///
    /// Returns an error if name resolution fails.
    async fn lookup_srv(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = (String, u16)> + Send>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
#[cfg(feature = "resolver")]
impl NetworkResolver for Resolver {
    async fn lookup_ip(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = IpAddr> + Send>, Box<dyn Error + Send + Sync>> {
        match self.as_resolver().lookup_ip(name).await {
            Ok(lookup) => Ok(Box::new(lookup.into_iter())),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_ipv4(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv4Addr> + Send>, Box<dyn Error + Send + Sync>> {
        match self.as_resolver().ipv4_lookup(name).await {
            Ok(lookup) => Ok(Box::new(lookup.into_iter().map(|a| a.0))),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_ipv6(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv6Addr> + Send>, Box<dyn Error + Send + Sync>> {
        match self.as_resolver().ipv6_lookup(name).await {
            Ok(lookup) => Ok(Box::new(lookup.into_iter().map(|aaaa| aaaa.0))),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_ns(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = String> + Send>, Box<dyn Error + Send + Sync>> {
        match self.as_resolver().ns_lookup(name).await {
            Ok(lookup) => Ok(Box::new(lookup.into_iter().map(|ns| ns.to_utf8()))),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_srv(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = (String, u16)> + Send>, Box<dyn Error + Send + Sync>> {
        match self.as_resolver().srv_lookup(name).await {
            Ok(lookup) => {
                Ok(Box::new(lookup.into_iter().map(|srv| (srv.target().to_utf8(), srv.port()))))
            }
            Err(err) => Err(Box::new(err)),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "ureq")]
impl UreqResolver for DnsResolver {
    fn resolve(
        &self,
        uri: &Uri,
        _: &Config,
        _: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        use std::net::SocketAddr;

        let host = uri.host().ok_or_else(|| {
            ureq::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "URI is missing a host",
            )))
        })?;

        async_io::block_on(self.lookup_ip(host)).map_or_else(
            |err| Err(ureq::Error::Other(err)),
            |ips| {
                let port = uri.port_u16().unwrap_or_else(|| match uri.scheme() {
                    Some(https) if https.as_str() == "https" => 443,
                    None | Some(_) => 80,
                });

                let mut results = self.empty();
                ips.into_iter()
                    .for_each(|ip| results.push(SocketAddr::new(ip, port)));
                #[cfg(feature = "tracing")]
                tracing::trace!(target: "froglight_api::resolver::ureq", "Resolved \"{host}\" to {:?}", results.as_ref());

                Ok(results)
            },
        )
    }
}

#[cfg(all(feature = "ureq", feature = "resolver"))]
impl UreqResolver for Resolver {
    fn resolve(
        &self,
        uri: &Uri,
        config: &Config,
        _: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        use std::net::SocketAddr;

        use async_io::block_on;
        use ureq::config::IpFamily;

        let host = uri.host().ok_or_else(|| {
            ureq::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "URI is missing a host",
            )))
        })?;

        let mut results = self.empty();
        let port = uri.port_u16().unwrap_or_else(|| match uri.scheme() {
            Some(https) if https.as_str() == "https" => 443,
            None | Some(_) => 80,
        });

        match config.ip_family() {
            IpFamily::Any => match block_on(self.as_resolver().lookup_ip(host)) {
                Ok(lookup) => {
                    lookup.into_iter().for_each(|ip| results.push(SocketAddr::new(ip, port)));
                }
                Err(err) => Err(ureq::Error::Other(Box::new(err)))?,
            },
            IpFamily::Ipv4Only => match block_on(self.as_resolver().ipv4_lookup(uri.to_string())) {
                Ok(lookup) => lookup
                    .into_iter()
                    .for_each(|a| results.push(SocketAddr::new(IpAddr::V4(a.0), port))),
                Err(err) => Err(ureq::Error::Other(Box::new(err)))?,
            },
            IpFamily::Ipv6Only => match block_on(self.as_resolver().ipv6_lookup(uri.to_string())) {
                Ok(lookup) => lookup
                    .into_iter()
                    .for_each(|aaaa| results.push(SocketAddr::new(IpAddr::V6(aaaa.0), port))),
                Err(err) => Err(ureq::Error::Other(Box::new(err)))?,
            },
        }

        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::resolver::ureq", "Resolved \"{host}\" to {:?}", results.as_ref());

        Ok(results)
    }
}

#[cfg(feature = "reqwest")]
impl ReqwestResolve for Resolver {
    fn resolve(&self, name: Name) -> Resolving {
        use std::{error::Error, net::SocketAddr};

        let resolver = self.clone();
        Box::pin(async move {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_api::resolver::reqwest", "Resolving IP for \"{}\"", name.as_str());

            resolver.as_resolver().lookup_ip(name.as_str().to_string()).await.map_or_else(
                |err| -> Result<_, Box<dyn Error + Send + Sync>> { Err(Box::new(err)) },
                |val| -> Result<Box<dyn Iterator<Item = SocketAddr> + Send>, _> {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(target: "froglight_api::resolver::reqwest", "Resolved \"{}\" to IPs {:?}", name.as_str(), val.as_lookup().records());

                    Ok(Box::new(val.into_iter().map(move |ip| SocketAddr::new(ip, 0))))
                },
            )
        })
    }
}

#[cfg(all(feature = "reqwest", feature = "resolver"))]
impl ReqwestResolve for DnsResolver {
    fn resolve(&self, name: Name) -> Resolving {
        use std::net::SocketAddr;

        let resolver = self.clone();
        Box::pin(async move {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_api::resolver::reqwest", "Resolving IP for \"{}\"", name.as_str());

            resolver.lookup_ip(name.as_str()).await.map(
                |val| -> Box<dyn Iterator<Item = SocketAddr> + Send> {
                    Box::new(val.into_iter().map(move |ip| SocketAddr::new(ip, 0)))
                },
            )
        })
    }
}
