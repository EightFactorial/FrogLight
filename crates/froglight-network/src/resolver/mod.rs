//! A `FrogLight` plugin for resolving domain names to IP addresses.

use std::sync::Mutex;

pub use async_std_resolver::config::{
    LookupIpStrategy, ResolverConfig, ResolverOpts, ServerOrderingStrategy,
};
use async_std_resolver::AsyncStdResolver;
use bevy_app::{App, Plugin};
use bevy_log::debug;

mod resource;
pub use resource::{
    Resolver, ResolverIpTask, ResolverIpv4Task, ResolverIpv6Task, ResolverServerTask,
    ResolverSrvTask,
};

/// A [`Plugin`] that resolves domain names to IP addresses.
///
/// By default, this plugin uses Cloudflare's DNS service
/// and prefers IPv4 addresses over IPv6 addresses.
///
/// # Example
/// ```rust,no_run
/// use froglight_network::ResolverPlugin;
///
/// // Use cloudflare and prefer IPv4 addresses
/// let cloudflare_ipv4_prefer = ResolverPlugin::default();
///
/// // Use quad9 and prefer IPv6 addresses
/// let quad9_ipv6_prefer = ResolverPlugin::quad9().prefer_ipv6();
///
/// // Use google and only resolve IPv4 addresses
/// let google_ipv4_only = ResolverPlugin::google().ipv4_only();
///
/// // Use cloudflare and only resolve IPv6 addresses
/// let cloudflare_ipv6_only = ResolverPlugin::cloudflare().ipv6_only();
/// ```
#[derive(Debug)]
pub struct ResolverPlugin {
    config: Mutex<ResolverConfig>,
    opts: Mutex<ResolverOpts>,
}

impl Default for ResolverPlugin {
    fn default() -> Self { Self::cloudflare() }
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

impl Plugin for ResolverPlugin {
    fn build(&self, app: &mut App) {
        debug!("Creating new ResolverResource");

        // Get the config and opts from the plugin
        let config = std::mem::take(&mut *self.config.lock().unwrap());
        let options = std::mem::take(&mut *self.opts.lock().unwrap());

        // Create the resolver client and insert it into the app
        #[allow(clippy::default_trait_access)]
        let resolver = AsyncStdResolver::new(config, options, Default::default());
        app.insert_resource(Resolver::new(resolver));
    }
}
