use bevy::app::{PluginGroup, PluginGroupBuilder};

/// A [`PluginGroup`] containing all client plugins.
///
/// This includes:
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder { PluginGroupBuilder::start::<Self>() }
}
