//! TODO

use std::{fmt::Debug, future::Future, sync::Arc};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "system-config")]
use hickory_resolver::system_conf::read_system_conf;
use hickory_resolver::{
    IntoName, ResolveError,
    config::{ResolverConfig, ResolverOpts},
    lookup::{SrvLookup, TxtLookup},
    lookup_ip::LookupIp,
    proto::runtime::Executor,
};

use super::{FroglightInnerResolver, ResolverConnectionProvider};

/// A resolver for server addresses.
///
/// This resolver is cheaply cloneable and can be shared between threads.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, Resource))]
pub struct FroglightResolver {
    resolver: Arc<FroglightInnerResolver>,
}

impl FroglightResolver {
    /// Create a new [`FroglightResolver`].
    ///
    /// See [`ResolverConfig`] on how to configure the resolver.
    #[must_use]
    pub fn new(config: ResolverConfig, options: ResolverOpts) -> Self {
        let mut resolver =
            FroglightInnerResolver::builder_with_config(config, ResolverConnectionProvider::new());
        *resolver.options_mut() = options;
        Self { resolver: Arc::new(resolver.build()) }
    }

    /// Create a new [`FroglightResolver`] from the system configuration.
    ///
    /// # Errors
    /// Returns an error if the system configuration could not be read.
    #[cfg(feature = "system-config")]
    pub fn system_config() -> Result<Self, std::io::Error> {
        let (config, options) = read_system_conf()?;
        Ok(Self::new(config, options))
    }

    /// Create a new [`FroglightResolver`] that uses Cloudflare's DNS servers.
    #[must_use]
    pub fn cloudflare() -> Self { Self::new(ResolverConfig::cloudflare(), ResolverOpts::default()) }

    /// Create a new [`FroglightResolver`] from the system configuration,
    /// falling back to using Cloudflare's DNS servers if it cannot be read.
    #[must_use]
    #[cfg(feature = "system-config")]
    pub fn system_or_cloudflare() -> Self {
        Self::system_config().unwrap_or_else(|_| Self::cloudflare())
    }

    /// Lookup an IP address for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::lookup_ip`] for more information.
    pub fn lookup_ip<'a, N: IntoName + Debug + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<LookupIp, ResolveError>> + 'a {
        #[cfg(feature = "trace")]
        tracing::trace!(target: "froglight_resolver", "Looking up IP for: {host:?}");
        self.resolver.lookup_ip(host)
    }

    /// Lookup SRV records for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::srv_lookup`] for more
    /// information.
    pub fn lookup_srv<'a, N: IntoName + Debug + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<SrvLookup, ResolveError>> + 'a {
        #[cfg(feature = "trace")]
        tracing::trace!(target: "froglight_resolver", "Looking up SRV for: {host:?}");
        self.resolver.srv_lookup::<N>(host)
    }

    /// Lookup TXT records for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::txt_lookup`] for more
    /// information.
    pub fn lookup_txt<'a, N: IntoName + Debug + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<TxtLookup, ResolveError>> + 'a {
        #[cfg(feature = "trace")]
        tracing::trace!(target: "froglight_resolver", "Looking up TXT for: {host:?}");
        self.resolver.txt_lookup::<N>(host)
    }
}

#[cfg(feature = "bevy")]
#[allow(unused_variables)]
impl bevy_ecs::world::FromWorld for FroglightResolver {
    #[cfg(feature = "system-config")]
    fn from_world(_: &mut bevy_ecs::world::World) -> Self {
        Self::system_config().unwrap_or_else(|err| {
            #[cfg(feature = "trace")]
            tracing::error!(target: "froglight_resolver", "Failed to load system resolver, defaulting to Cloudflare: {err}");
            Self::cloudflare()
        })
    }

    #[cfg(not(feature = "system-config"))]
    fn from_world(_: &mut bevy_ecs::world::World) -> Self { Self::cloudflare() }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
