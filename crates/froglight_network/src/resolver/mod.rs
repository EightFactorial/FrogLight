//! TODO

use alloc::sync::Arc;
use core::{
    fmt::Debug,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use std::sync::OnceLock;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use hickory_resolver::{
    IntoName, ResolveError,
    lookup::{SrvLookup, TxtLookup},
    lookup_ip::LookupIp,
};

mod hickory;
pub use hickory::ResolverProvider;
pub(crate) use hickory::RuntimeExecutor;
pub use hickory_resolver::Resolver;

#[cfg(feature = "ureq")]
mod ureq;
#[cfg(feature = "ureq")]
pub use ureq::HttpClient;

/// A DNS resolver.
///
/// Uses a shared [`hickory_resolver::Resolver`] under the hood.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Resource, opaque))]
pub struct DnsResolver {
    provider: Arc<Resolver<ResolverProvider>>,
}

impl Debug for DnsResolver {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DnsResolver").finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

static INSTANCE: OnceLock<DnsResolver> = OnceLock::new();

impl DnsResolver {
    /// Get the global [`DnsResolver`].
    ///
    /// # Panics
    ///
    /// Panics if the global instance has not been initialized yet.
    #[must_use]
    pub fn get() -> &'static Self {
        INSTANCE.get().expect("The `DnsResolver` has not been initialized yet!")
    }

    /// Attempts to get the global [`DnsResolver`],
    /// or returns `None` if it is not initialized.
    #[must_use]
    pub fn try_get() -> Option<&'static Self> { INSTANCE.get() }

    /// Gets the global [`DnsResolver`] instance, or initializes it with `f`.
    #[must_use]
    pub fn get_or_init(f: impl FnOnce() -> Resolver<ResolverProvider>) -> &'static Self {
        INSTANCE.get_or_init(|| DnsResolver { provider: Arc::new(f()) })
    }

    /// Gets the global [`DnsResolver`] instance,
    /// or initializes it with the defaults.
    #[must_use]
    pub fn get_or_default() -> &'static Self {
        use hickory_resolver::config::ResolverConfig;

        #[cfg(all(feature = "resolver-sys", any(unix, target_os = "windows")))]
        fn default() -> Resolver<ResolverProvider> {
            match hickory_resolver::system_conf::read_system_conf() {
                // Use the configuration from the system.
                Ok((config, options)) => {
                    let mut builder =
                        Resolver::builder_with_config(config, ResolverProvider::default());
                    *builder.options_mut() = options;

                    builder.build()
                }
                // Or fallback to Cloudflare's public DNS and default options.
                Err(..) => Resolver::builder_with_config(
                    ResolverConfig::cloudflare(),
                    ResolverProvider::default(),
                )
                .build(),
            }
        }

        #[cfg(not(all(feature = "resolver-sys", any(unix, target_os = "windows"))))]
        fn default() -> Resolver<ResolverProvider> {
            Resolver::builder_with_config(ResolverConfig::cloudflare(), ResolverProvider::default())
                .build()
        }

        Self::get_or_init(default)
    }
}

// -------------------------------------------------------------------------------------------------

#[expect(clippy::missing_errors_doc, reason = "Has links to the relevant docs")]
impl DnsResolver {
    /// Lookup an IP address for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::lookup_ip`] for more information.
    #[inline]
    pub async fn lookup_ip<N: IntoName + Debug>(&self, host: N) -> Result<LookupIp, ResolveError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Resolving IP for host: {host:?}");

        self.provider.lookup_ip(host).await
    }

    /// Lookup TXT records for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::txt_lookup`] for more
    /// information.
    #[inline]
    pub async fn lookup_txt<N: IntoName + Debug>(
        &self,
        host: N,
    ) -> Result<TxtLookup, ResolveError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Resolving TXT for host: {host:?}");

        self.provider.txt_lookup(host).await
    }

    /// Lookup SRV records for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::srv_lookup`] for more
    /// information.
    #[inline]
    pub async fn lookup_srv<N: IntoName + Debug>(
        &self,
        host: N,
    ) -> Result<SrvLookup, ResolveError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Resolving SRV for host: {host:?}");

        self.provider.srv_lookup(host).await
    }

    /// Lookup the server address for a given hostname,
    /// using special SRV records or falling back to A/AAAA records.
    ///
    /// # Errors
    ///
    /// Returns an error if the hostname could not be resolved,
    /// or if an error occurred during the resolution process.
    pub async fn lookup_minecraft<N: AsRef<str> + ?Sized>(
        &self,
        host: &N,
    ) -> Result<SocketAddr, ResolveError> {
        let mut address = host.as_ref();
        let mut port = 25565;

        #[cfg(feature = "tracing")]
        tracing::debug!("Resolving MC for host: {address:?}");

        if let Ok(addr) = SocketAddr::from_str(address) {
            return Ok(addr);
        } else if let Ok(addr) = IpAddr::from_str(address) {
            return Ok(SocketAddr::new(addr, port));
        }

        // Split off a port if one is present
        if let Some((addr, pt)) = address.rsplit_once(':')
            && pt.chars().all(|c| c.is_ascii_digit())
            && let Ok(pt) = u16::from_str(pt)
        {
            address = addr;
            port = pt;
        }

        // Prepare the server address
        let name = address.into_name()?;

        // Prepend a `_minecraft._tcp` label and lookup any SRV records
        if let Ok(name) = name.prepend_label("_minecraft._tcp")
            && let Ok(lookup) = self.lookup_srv(name).await
        {
            for record in lookup {
                // Lookup the target of the SRV record
                let Ok(lookup) = self.lookup_ip(record.target().clone()).await else {
                    continue;
                };

                // Use the first IP address found
                if let Some(ip) = lookup.into_iter().next() {
                    return Ok(SocketAddr::new(ip, port));
                }
            }
        }

        // Otherwise, use the address for A/AAAA records
        if let Some(ip) = self.lookup_ip(name).await?.into_iter().next() {
            return Ok(SocketAddr::new(ip, port));
        }

        Err(ResolveError::from("could not resolve address into an IP address"))
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "bevy")]
impl bevy_ecs::world::FromWorld for DnsResolver {
    fn from_world(world: &mut World) -> Self {
        world.get_resource::<Self>().unwrap_or_else(|| Self::get_or_default()).clone()
    }
}
