//! [`ResolverPlugin`] for Bevy applications.

use bevy_app::prelude::*;
use derive_more::Deref;

use crate::prelude::*;

/// A [`Plugin`] that adds a [`FroglightResolver`] to an [`App`].
///
/// If the `agent` feature is enabled, it also adds a [`FroglightAgent`].
///
/// By default uses the system configuration for DNS resolution,
/// but falls back to Cloudflare if the system configuration cannot be read.
#[derive(Debug, Default, Clone, Deref)]
pub struct ResolverPlugin(Option<FroglightResolver>);

impl ResolverPlugin {
    /// Create a new [`ResolverPlugin`].
    ///
    /// See [`ResolverConfig`] and [`ResolverOpts`] for details.
    #[must_use]
    pub fn new(config: ResolverConfig, options: ResolverOpts) -> Self {
        Self(Some(FroglightResolver::new(config, options)))
    }

    /// Create a new [`ResolverPlugin`] using a [`FroglightResolver`].
    #[inline]
    #[must_use]
    pub const fn from_resolver(resolver: FroglightResolver) -> Self { Self(Some(resolver)) }
}

// -------------------------------------------------------------------------------------------------

impl Plugin for ResolverPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "agent")]
        app.register_type::<FroglightAgent>();
        app.register_type::<FroglightResolver>();

        // Use the provided resolver, an existing resolver, or create a new one.
        let resolver = self.0.clone();
        let resolver = resolver
            .unwrap_or_else(|| app.world_mut().get_resource_or_init::<FroglightResolver>().clone());

        // Insert an agent resource if the feature is enabled.
        #[cfg(feature = "agent")]
        app.insert_resource(FroglightAgent::new(&resolver));

        // If the resolver was provided, insert it into the app.
        self.0.is_some().then(|| app.insert_resource(resolver));
    }
}
