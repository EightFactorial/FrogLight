//! TODO

use std::{future::Future, sync::Arc};

pub use hickory_resolver::{
    config::{
        LookupIpStrategy, NameServerConfig, NameServerConfigGroup, ResolverConfig, ResolverOpts,
    },
    lookup::{Lookup, SrvLookup, TxtLookup},
    lookup_ip::LookupIp,
    IntoName, Name, TryParseIp,
};
use hickory_resolver::{error::ResolveError, proto::Executor};

mod provider;
use provider::{FroglightResolver, ResolverConnectionProvider};

/// A resolver for server addresses.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::system::Resource))]
pub struct ServerResolver {
    resolver: Arc<FroglightResolver>,
}

impl ServerResolver {
    /// Create a new [`ServerResolver`].
    #[must_use]
    pub fn new(config: ResolverConfig, options: ResolverOpts) -> Self {
        let resolver = FroglightResolver::new(config, options, ResolverConnectionProvider::new());

        Self { resolver: Arc::new(resolver) }
    }

    /// Create a new [`ServerResolver`] from the system configuration.
    ///
    /// # Errors
    /// Returns an error if the system configuration could not be read.
    pub fn system_config() -> Result<Self, std::io::Error> {
        let resolver = FroglightResolver::from_system_conf(ResolverConnectionProvider::new())?;

        Ok(Self { resolver: Arc::new(resolver) })
    }

    /// Lookup an IP address for a given hostname.
    pub fn lookup_ip<'a, N: IntoName + TryParseIp + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<LookupIp, ResolveError>> + 'a {
        self.resolver.lookup_ip::<N>(host)
    }

    /// Lookup SRV records for a given hostname.
    pub fn lookup_srv<'a, N: IntoName + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<SrvLookup, ResolveError>> + 'a {
        self.resolver.srv_lookup::<N>(host)
    }

    /// Lookup TXT records for a given hostname.
    pub fn lookup_txt<'a, N: IntoName + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<TxtLookup, ResolveError>> + 'a {
        self.resolver.txt_lookup::<N>(host)
    }
}

#[cfg(feature = "bevy")]
impl bevy_ecs::world::FromWorld for ServerResolver {
    fn from_world(_: &mut bevy_ecs::world::World) -> Self {
        Self::system_config().unwrap_or_else(|err| {
            bevy_log::warn!("Failed to load system resolver, defaulting to cloudflare: {err}");
            Self::new(ResolverConfig::cloudflare(), ResolverOpts::default())
        })
    }
}
