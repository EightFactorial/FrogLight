use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] containing all basic `FrogLight` plugins.
///
/// This includes:
/// - [`BlockPlugin`]
/// - [`NetworkPlugins`]
/// - [`UtilityPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicPlugins;

impl PluginGroup for BasicPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(NetworkPlugins);
        builder.add(BlockPlugin).add(UtilityPlugin)
    }
}
