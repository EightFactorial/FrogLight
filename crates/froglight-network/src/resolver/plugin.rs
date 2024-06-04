//! Resolver

use async_std_resolver::config::{LookupIpStrategy, ResolverConfig, ResolverOpts};
use bevy_app::{App, Plugin};
use parking_lot::Mutex;

use super::Resolver;

/// A DNS resolver plugin.
///
/// Uses the [`async_std_resolver`](https://crates.io/crates/async-std-resolver) crate.
///
/// # Example
/// ```rust,no_run
/// use froglight_network::resolver::ResolverPlugin;
///
/// // By default the resolver prefers IPv4 over IPv6.
/// let _cloudflare_prefer_ipv4 = ResolverPlugin::default();
/// let _google_prefer_ipv4 = ResolverPlugin::default().google();
/// let _quad9_prefer_ipv4 = ResolverPlugin::default().quad9();
///
/// // Use the `prefer_ipv6` method to prefer IPv6.
/// let _cloudflare_prefer_ipv6 = ResolverPlugin::default().prefer_ipv6();
///
/// // Use the `ipv4_only` method to only use IPv4.
/// let _google_ipv4_only = ResolverPlugin::default().google().ipv4_only();
///
/// // Use the `ipv6_only` method to only use IPv6.
/// let _quad9_ipv6_only = ResolverPlugin::default().quad9().ipv6_only();
/// ```
#[derive(Debug)]
pub struct ResolverPlugin {
    config: Mutex<ResolverConfig>,
    opts: Mutex<ResolverOpts>,
}

impl Default for ResolverPlugin {
    /// Create a new [`ResolverPlugin`] with the default config.
    ///
    /// Uses Cloudflare's DNS service and prefers IPv4 over IPv6.
    fn default() -> Self { Self::new(ResolverConfig::cloudflare()).prefer_ipv4() }
}

impl Plugin for ResolverPlugin {
    fn build(&self, _: &mut App) {}

    fn finish(&self, app: &mut App) {
        // Get the config and opts from the plugin
        let config = std::mem::take(&mut *self.config.lock());
        let options = std::mem::take(&mut *self.opts.lock());

        // Create and insert a new resolver
        app.insert_resource(Resolver::build(config, options));
    }
}

impl ResolverPlugin {
    /// Create a new [`ResolverPlugin`] with the given config.
    ///
    /// Use this if you want to pass in your own [`ResolverConfig`].
    #[must_use]
    pub fn new(config: ResolverConfig) -> Self {
        Self { config: Mutex::new(config), opts: Mutex::new(ResolverOpts::default()) }
    }

    /// Set the [`ResolverConfig`] for the resolver.
    ///
    /// Use this if you want to overwrite the existing [`ResolverConfig`].
    #[must_use]
    pub fn with_config(self, config: ResolverConfig) -> Self {
        *self.config.lock() = config;
        self
    }

    /// Set the [`ResolverConfig`] to use Cloudflare's DNS service.
    ///
    /// See [`ResolverConfig::cloudflare`] for more information.
    ///
    /// This is the default resolver.
    #[must_use]
    #[inline]
    pub fn cloudflare(self) -> Self { self.with_config(ResolverConfig::cloudflare()) }

    /// Set the [`ResolverConfig`] to use Google's DNS service.
    ///
    /// See [`ResolverConfig::google`] for more information.
    #[must_use]
    #[inline]
    pub fn google(self) -> Self { self.with_config(ResolverConfig::google()) }

    /// Create a new resolver plugin using Quad9's DNS service.
    ///
    /// See [`ResolverConfig::quad9`] for more information.
    #[must_use]
    #[inline]
    pub fn quad9(self) -> Self { self.with_config(ResolverConfig::quad9()) }

    /// Set the options for the resolver.
    ///
    /// Use this if you want to overwrite the existing [`ResolverOpts`].
    #[must_use]
    pub fn with_opts(self, opts: ResolverOpts) -> Self {
        *self.opts.lock() = opts;
        self
    }

    /// Set the [`LookupIpStrategy`] for the resolver.
    ///
    /// Use this if you want to overwrite the existing [`LookupIpStrategy`].
    #[must_use]
    pub fn with_strategy(self, strategy: LookupIpStrategy) -> Self {
        self.opts.lock().ip_strategy = strategy;
        self
    }

    /// Set the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv4Only`].
    #[must_use]
    #[inline]
    pub fn ipv4_only(self) -> Self { self.with_strategy(LookupIpStrategy::Ipv4Only) }

    /// Set the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv6Only`].
    #[must_use]
    #[inline]
    pub fn ipv6_only(self) -> Self { self.with_strategy(LookupIpStrategy::Ipv6Only) }

    /// Set the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv6thenIpv4`].
    #[must_use]
    #[inline]
    pub fn prefer_ipv6(self) -> Self { self.with_strategy(LookupIpStrategy::Ipv6thenIpv4) }

    /// Set the [`LookupIpStrategy`] to [`LookupIpStrategy::Ipv4thenIpv6`].
    ///
    /// This is the default behavior.
    #[must_use]
    #[inline]
    pub fn prefer_ipv4(self) -> Self { self.with_strategy(LookupIpStrategy::Ipv4thenIpv6) }
}
