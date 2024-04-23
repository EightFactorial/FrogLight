use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] for all plugins that are part of the basic `FrogLight`
/// application.
///
/// This includes:
/// - [`CorePlugin`]
/// - [`EntityPlugin`]
/// - [`RegistryPlugin`]
/// - [`UtilityPlugin`]
/// - [`NetworkPlugins`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicPlugins;

impl PluginGroup for BasicPlugins {
    fn build(self) -> PluginGroupBuilder { Self::add(PluginGroupBuilder::start::<Self>()) }
}

impl BasicPlugins {
    /// Adds all the [`Plugins`](bevy::prelude::Plugin) that are part of the
    /// [`BasicPlugins`] [`PluginGroup`].
    pub(crate) fn add(builder: PluginGroupBuilder) -> PluginGroupBuilder {
        builder
            .add(CorePlugin)
            .add(EntityPlugin)
            .add(RegistryPlugin)
            .add(UtilityPlugin)
            .add(NetworkPlugins)
    }
}
