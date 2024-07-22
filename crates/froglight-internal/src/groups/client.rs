use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] containing all client plugins.
///
/// This includes:
/// - [`AssetPlugins`]
/// - [`InterfacePlugins`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>();
        builder.add_group(AssetPlugins).add_group(InterfacePlugins)
    }
}
