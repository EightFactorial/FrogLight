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

#[cfg(feature = "resolver")]
pub mod hickory;
#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "ureq")]
mod ureq;

/// A DNS resolver for performing record lookups.
///
/// Supports multiple underlying implementations via the [`NetworkResolver`]
/// trait.
///
/// ## Note
///
/// This type is thread-safe and can be cloned cheaply.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, Resource))]
#[cfg_attr(all(feature = "bevy", feature = "resolver"), reflect(Default))]
pub struct DnsResolver(Arc<dyn NetworkResolver>);

#[cfg(feature = "resolver")]
impl Default for DnsResolver {
    #[inline]
    fn default() -> Self { Self::new(hickory::Resolver::default()) }
}

impl DnsResolver {
    /// Creates a new [`DnsResolver`] from a [`NetworkResolver`].
    #[inline]
    #[must_use]
    pub fn new<T: NetworkResolver>(agent: T) -> Self { Self::new_arc(Arc::new(agent)) }

    /// Creates a new [`DnsResolver`] from an [`Arc<dyn NetworkResolver>`].
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
    pub fn spawn_task_using<F: FnOnce(Self) -> Fut, Fut: Future<Output = ()> + Send + 'static>(
        &self,
        f: F,
    ) {
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
impl NetworkResolver for hickory::Resolver {
    async fn lookup_ip(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = IpAddr> + Send>, Box<dyn Error + Send + Sync>> {
        match self.as_resolver().lookup_ip(name).await {
            Ok(lookup) => Ok(Box::new(
                lookup
                    .as_lookup()
                    .message()
                    .answers
                    .clone()
                    .into_iter()
                    .filter_map(|record| record.data.ip_addr()),
            )),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_ipv4(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv4Addr> + Send>, Box<dyn Error + Send + Sync>> {
        use hickory_resolver::proto::rr::RData;

        match self.as_resolver().ipv4_lookup(name).await {
            Ok(lookup) => {
                Ok(Box::new(lookup.message().answers.clone().into_iter().filter_map(|record| {
                    if let RData::A(a) = record.data { Some(a.0) } else { None }
                })))
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_ipv6(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = Ipv6Addr> + Send>, Box<dyn Error + Send + Sync>> {
        use hickory_resolver::proto::rr::RData;

        match self.as_resolver().ipv6_lookup(name).await {
            Ok(lookup) => {
                Ok(Box::new(lookup.message().answers.clone().into_iter().filter_map(|record| {
                    if let RData::AAAA(aaaa) = record.data { Some(aaaa.0) } else { None }
                })))
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_ns(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = String> + Send>, Box<dyn Error + Send + Sync>> {
        use hickory_resolver::proto::rr::RData;

        match self.as_resolver().ns_lookup(name).await {
            Ok(lookup) => {
                Ok(Box::new(lookup.message().answers.clone().into_iter().filter_map(|record| {
                    if let RData::NS(ns) = &record.data { Some(ns.to_utf8()) } else { None }
                })))
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn lookup_srv(
        &self,
        name: &str,
    ) -> Result<Box<dyn Iterator<Item = (String, u16)> + Send>, Box<dyn Error + Send + Sync>> {
        use hickory_resolver::proto::rr::RData;

        match self.as_resolver().srv_lookup(name).await {
            Ok(lookup) => {
                Ok(Box::new(lookup.message().answers.clone().into_iter().filter_map(|record| {
                    if let RData::SRV(srv) = &record.data {
                        Some((srv.target.to_utf8(), srv.port))
                    } else {
                        None
                    }
                })))
            }
            Err(err) => Err(Box::new(err)),
        }
    }
}
