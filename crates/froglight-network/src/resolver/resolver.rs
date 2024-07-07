use std::{net::SocketAddr, sync::Arc};

use async_std_resolver::{
    config::{ResolverConfig, ResolverOpts},
    AsyncStdResolver, ResolveError,
};
use bevy_ecs::system::Resource;

/// A DNS resolver.
///
/// Used to asynchronously resolve domain names to IP addresses
/// and lookup SRV records.
#[derive(Debug, Clone, Resource)]
pub struct Resolver {
    resolver: Arc<AsyncStdResolver>,
}

impl Resolver {
    /// Creates a new resolver from the given [`AsyncStdResolver`]
    #[must_use]
    pub fn new(resolver: AsyncStdResolver) -> Self { Self { resolver: Arc::new(resolver) } }

    /// Builds a new resolver from the given [`ResolverConfig`] and
    /// [`ResolverOpts`].
    #[allow(clippy::default_trait_access)]
    #[must_use]
    pub fn build(config: ResolverConfig, opts: ResolverOpts) -> Self {
        Self::new(AsyncStdResolver::new(config, opts, Default::default()))
    }

    /// Returns a reference to the underlying [`AsyncStdResolver`].
    #[must_use]
    #[inline]
    pub fn resolver(&self) -> &AsyncStdResolver { &self.resolver }

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
    pub async fn lookup_mc<'a>(
        &'a self,
        mut address: &'a str,
    ) -> Result<Option<SocketAddr>, ResolveError> {
        let mut port: u16 = 25565;
        if let Some((split_host, split_port)) = address.split_once(':') {
            // Parse the port
            if let Ok(split_port) = split_port.parse::<u16>() {
                port = split_port;
            }

            // Update the address
            address = split_host;
        }

        // Lookup a SRV record for the given address
        self.lookup_srv(address, port).await
    }

    /// Looks up an SRV record for the given address.
    ///
    /// # Errors
    /// If an error occurs while resolving the address.
    pub async fn lookup_srv<'a>(
        &'a self,
        address: &'a str,
        port: u16,
    ) -> Result<Option<SocketAddr>, ResolveError> {
        let srv_address = format!("_minecraft._tcp.{address}");
        let Ok(result) = self.resolver.srv_lookup(srv_address).await else {
            // No SRV record found, fallback to A/AAAA
            return self.lookup_ip(address, port).await;
        };

        match (result.iter().next(), result.ip_iter().next()) {
            (Some(srv), Some(ip)) => Ok(Some(SocketAddr::new(ip, srv.port()))),
            (None, Some(ip)) => Ok(Some(SocketAddr::new(ip, port))),
            (Some(srv), None) => self.lookup_ip(&srv.target().to_string(), srv.port()).await,
            (None, None) => self.lookup_ip(address, port).await,
        }
    }

    /// Looks up an A/AAAA record for the given address.
    ///
    /// # Errors
    /// If an error occurs while resolving the address.
    pub async fn lookup_ip<'a>(
        &'a self,
        address: &'a str,
        port: u16,
    ) -> Result<Option<SocketAddr>, ResolveError> {
        let ip_address = self.resolver.lookup_ip(address).await?.iter().next();
        Ok(ip_address.map(|ip| SocketAddr::new(ip, port)))
    }
}
