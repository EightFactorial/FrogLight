use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] for all plugins that are part of the basic `FrogLight`
/// application.
///
/// This includes:
/// - [`BlockPlugin`]
/// - `EntityPlugin`
/// - [`NetworkPlugins`]
/// - [`RegistryPlugin`]
/// - [`UtilityPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicPlugins;

impl PluginGroup for BasicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BlockPlugin)
            .add(NetworkPlugins)
            .add(RegistryPlugin)
            .add(UtilityPlugin)
    }
}
