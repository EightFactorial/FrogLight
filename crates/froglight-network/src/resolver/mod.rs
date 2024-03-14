//! A `FrogLight` plugin for resolving domain names to IP addresses.

use std::sync::Mutex;

pub use async_std_resolver::config::ResolverConfig;
use async_std_resolver::{config::ResolverOpts, AsyncStdResolver};
use bevy_app::{App, Plugin};

mod resource;
use bevy_log::debug;
pub use resource::{ResolverResource, ResolverSrvTask};

/// A [`Plugin`] that resolves domain names to IP addresses.
///
/// By default, this plugin uses Cloudflare's DNS service.
#[derive(Debug)]
pub struct ResolverPlugin {
    pub(crate) config: Mutex<ResolverConfig>,
    pub(crate) opts: Mutex<ResolverOpts>,
}

impl Default for ResolverPlugin {
    fn default() -> Self { Self::new(ResolverConfig::cloudflare()) }
}

impl ResolverPlugin {
    /// Creates a new resolver plugin with the given config.
    #[must_use]
    pub fn new(config: ResolverConfig) -> Self {
        Self { config: Mutex::new(config), opts: Mutex::new(ResolverOpts::default()) }
    }
}

impl Plugin for ResolverPlugin {
    fn build(&self, app: &mut App) {
        debug!("Creating new ResolverResource");

        // Get the config and opts from the plugin
        let config = std::mem::take(&mut *self.config.lock().unwrap());
        let opts = std::mem::take(&mut *self.opts.lock().unwrap());

        // Create the resolver client and insert it into the app
        #[allow(clippy::default_trait_access)]
        let client = AsyncStdResolver::new(config, opts, Default::default());
        app.insert_resource(ResolverResource::new(client));
    }
}
