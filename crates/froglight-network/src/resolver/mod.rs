//! Resolver

use std::sync::Mutex;

pub use async_std_resolver::{
    config::{LookupIpStrategy, ResolverConfig, ResolverOpts},
    AsyncStdResolver, ResolveError as AsyncStdResolveError,
};
use bevy::app::{App, Plugin};
pub use tldextract::{TldExtractError, TldExtractor, TldOption};

mod resource;
pub use resource::{Resolver, ResolverError};

/// A DNS resolver plugin.
///
/// Uses the [`async_std_resolver`](https://docs.rs/async-std-resolver) crate.
///
/// # Example
/// ```rust,no_run
/// use froglight_network::resolver::ResolverPlugin;
///
/// // By default the resolver prefers IPv4.
/// let cloudflare_prefer_ipv4 = ResolverPlugin::default();
/// let google_prefer_ipv4 = ResolverPlugin::google();
/// let quad9_prefer_ipv4 = ResolverPlugin::quad9();
///
/// // Use the `ipv6_only` method to prefer IPv6.
/// let cloudflare_prefer_ipv6 = ResolverPlugin::cloudflare().prefer_ipv6();
///
/// // Use the `ipv4_only` method to only use IPv4.
/// let google_ipv4_only = ResolverPlugin::google().ipv4_only();
///
/// // Use the `ipv6_only` method to only use IPv6.
/// let quad9_ipv6_only = ResolverPlugin::quad9().ipv6_only();
///
/// // etc...
/// ```
#[derive(Debug)]
pub struct ResolverPlugin {
    config: Mutex<ResolverConfig>,
    opts: Mutex<ResolverOpts>,
}

impl Default for ResolverPlugin {
    fn default() -> Self { Self::cloudflare() }
}

impl Plugin for ResolverPlugin {
    fn build(&self, app: &mut App) {
        // Get the config and opts from the plugin
        let config = std::mem::take(&mut *self.config.lock().unwrap());
        let options = std::mem::take(&mut *self.opts.lock().unwrap());

        // Create and insert a new resolver
        app.insert_resource(Resolver::build(config, options));
    }
}

impl ResolverPlugin {
    /// Creates a new resolver plugin with the given config.
    #[must_use]
    pub fn new(config: ResolverConfig) -> Self {
        Self { config: Mutex::new(config), opts: Mutex::new(ResolverOpts::default()) }
    }

    /// Sets the options for the resolver.
    #[must_use]
    pub fn with_opts(self, opts: ResolverOpts) -> Self { Self { opts: Mutex::new(opts), ..self } }

    /// Sets the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv4Only`].
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn ipv4_only(self) -> Self {
        {
            let mut opts = self.opts.lock().unwrap();
            opts.ip_strategy = LookupIpStrategy::Ipv4Only;
        }

        self
    }

    /// Sets the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv6Only`].
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn ipv6_only(self) -> Self {
        {
            let mut opts = self.opts.lock().unwrap();
            opts.ip_strategy = LookupIpStrategy::Ipv6Only;
        }

        self
    }

    /// Sets the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv4thenIpv6`].
    ///
    /// This is the default.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn prefer_ipv4(self) -> Self {
        {
            let mut opts = self.opts.lock().unwrap();
            opts.ip_strategy = LookupIpStrategy::Ipv4thenIpv6;
        }

        self
    }

    /// Sets the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv6thenIpv4`].
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn prefer_ipv6(self) -> Self {
        {
            let mut opts = self.opts.lock().unwrap();
            opts.ip_strategy = LookupIpStrategy::Ipv6thenIpv4;
        }

        self
    }

    /// Creates a new resolver plugin using Cloudflare's DNS service.
    ///
    /// See [`ResolverConfig::cloudflare`] for more information.
    ///
    /// This is the default.
    #[must_use]
    pub fn cloudflare() -> Self { Self::new(ResolverConfig::cloudflare()) }

    /// Creates a new resolver plugin using Google's DNS service.
    ///
    /// See [`ResolverConfig::google`] for more information.
    #[must_use]
    pub fn google() -> Self { Self::new(ResolverConfig::google()) }

    /// Creates a new resolver plugin using Quad9's DNS service.
    ///
    /// See [`ResolverConfig::quad9`] for more information.
    #[must_use]
    pub fn quad9() -> Self { Self::new(ResolverConfig::quad9()) }
}
