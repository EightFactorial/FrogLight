use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use async_std_resolver::{
    config::{ResolverConfig, ResolverOpts},
    AsyncStdResolver, ResolveError,
};
use bevy::{ecs::system::Resource, log::debug};
use thiserror::Error;
use tldextract::{TldExtractor, TldOption};

/// A DNS resolver.
///
/// Used to asynchronously resolve domain names to IP addresses
/// and lookup SRV records.
#[derive(Debug, Resource)]
pub struct Resolver {
    resolver: Arc<AsyncStdResolver>,
    extractor: Arc<TldExtractor>,
}

/// A resolver error.
#[derive(Debug, Error)]
pub enum ResolverError {
    /// An error occurred while resolving an address.
    #[error(transparent)]
    ResolveError(#[from] ResolveError),
    /// An error occurred while extracting the TLD from an address.
    #[error(transparent)]
    TldError(#[from] tldextract::TldExtractError),
}

impl Resolver {
    /// Creates a new resolver from the given [`AsyncStdResolver`] and
    /// [`TldExtractor`].
    #[must_use]
    pub fn new(resolver: AsyncStdResolver, extractor: TldExtractor) -> Self {
        Self { resolver: Arc::new(resolver), extractor: Arc::new(extractor) }
    }

    /// Builds a new resolver from the given [`ResolverConfig`] and
    /// [`ResolverOpts`].
    #[allow(clippy::default_trait_access)]
    #[must_use]
    pub fn build(config: ResolverConfig, opts: ResolverOpts) -> Self {
        Self::new(
            AsyncStdResolver::new(config, opts, Default::default()),
            TldOption::default().naive_mode(true).build(),
        )
    }

    /// Looks up the given address for an IP to connect to.
    ///
    /// If the address contains a port, it will be used as the port to connect
    /// to.
    ///
    /// This function will attempt to lookup an SRV record for the domain if it
    /// does *not* have a subdomain, otherwise it will lookup an A/AAAA
    /// record.
    ///
    /// # Examples:
    /// - `mc.hypixel.net` -> `A/AAAA`, port 25565
    /// - `mc.hypixel.net:25565` -> `A/AAAA`, port 25565
    /// - `mc.hypixel.net:25566` -> `A/AAAA`, port 25566
    /// - `hypixel.net` -> `SRV`, port 25565
    /// - `hypixel.net:25565` -> `SRV`, port 25565
    /// - `hypixel.net:25566` -> `SRV`, port 25566
    ///
    /// # Errors
    /// If the address is not a valid domain name.
    /// If an error occurs while resolving the address.
    pub async fn lookup_mc(&self, mut address: &str) -> Result<Option<SocketAddr>, ResolverError> {
        #[cfg(debug_assertions)]
        debug!("Looking for a server for `{address}`");

        let mut port: u16 = 25565;
        if let Some((split_host, split_port)) = address.split_once(':') {
            // Update the address
            address = split_host;
            // Parse the port
            if let Ok(split_port) = split_port.parse::<u16>() {
                port = split_port;
            }
        }

        // Extract the domain from the address
        let extracted = self.extractor.extract_naive(address)?;
        if let (None, Some(domain), Some(suffix)) =
            (&extracted.subdomain, &extracted.domain, &extracted.suffix)
        {
            // Lookup a SRV record for the given address
            let srv_address = format!("_minecraft._tcp.{domain}.{suffix}");
            if let Some(ip) = self.lookup_srv(&srv_address).await? {
                return Ok(Some(SocketAddr::new(ip, port)));
            }
        }

        // Lookup an A/AAAA record for the given address
        if let Some(ip) = self.lookup_ip(address).await? {
            Ok(Some(SocketAddr::new(ip, port)))
        } else {
            Ok(None)
        }
    }

    /// Looks up an SRV record for the given address.
    ///
    /// # Errors
    /// If an error occurs while resolving the address.
    pub async fn lookup_srv(&self, address: &str) -> Result<Option<IpAddr>, ResolveError> {
        let result = self.resolver.srv_lookup(address).await?.ip_iter().next();

        #[cfg(debug_assertions)]
        debug!("Found SRV for `{}`: `{:?}`", address, result);

        Ok(result)
    }

    /// Looks up an A/AAAA record for the given address.
    ///
    /// # Errors
    /// If an error occurs while resolving the address.
    pub async fn lookup_ip(&self, address: &str) -> Result<Option<IpAddr>, ResolveError> {
        let result = self.resolver.lookup_ip(address).await?.iter().next();

        #[cfg(debug_assertions)]
        debug!("Resolved `{}` to `{:?}`", address, result);

        #[cfg(debug_assertions)]
        Ok(result)
    }
}
