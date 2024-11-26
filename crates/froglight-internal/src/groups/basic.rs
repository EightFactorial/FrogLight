use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] containing all basic `FrogLight` plugins.
///
/// This includes:
/// - [`BlockPlugin`]
/// - [`EntityPlugin`]
/// - [`NetworkPlugins`]
/// - [`UtilityPlugin`]
/// - [`RegistryPlugin`] (if the `reflect` feature is enabled)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicPlugins;

impl PluginGroup for BasicPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add(BlockPlugin).add(EntityPlugin);
        builder = builder.add_group(NetworkPlugins).add(UtilityPlugin);

        #[cfg(feature = "reflect")]
        {
            builder = builder.add(RegistryPlugin);
        }

        builder
    }
}
