use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

use crate::prelude::plugins::*;

/// A [`PluginGroup`] containing all client plugins.
///
/// This includes:
/// - [`AssetPlugins`]
/// - [`InterfacePlugins`]
/// - [`RenderPlugin`]
/// - [`EntropyPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(AssetPlugins).add_group(InterfacePlugins);
        builder.add(RenderPlugin).add(EntropyPlugin::<WyRand>::new())
    }
}
