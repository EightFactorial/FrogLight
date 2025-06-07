//! [`ResolverPlugin`] for Bevy applications.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
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
        // Use the provided resolver or create a new one.
        let resolver =
            self.0.clone().unwrap_or_else(|| FroglightResolver::from_world(app.world_mut()));

        // Insert an agent resource if the feature is enabled.
        #[cfg(feature = "agent")]
        app.insert_resource(FroglightAgent::new(&resolver));
        #[cfg(feature = "agent")]
        app.register_type::<FroglightAgent>();

        // Insert the resolver resource.
        app.insert_resource(resolver);
        app.register_type::<FroglightResolver>();
    }
}
