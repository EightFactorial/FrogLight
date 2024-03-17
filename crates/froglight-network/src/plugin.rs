use bevy_app::{App, Plugin, PluginGroup, PluginGroupBuilder};

use crate::{resolver::ResolverPlugin, status::StatusPlugin};

/// The [`PluginGroup`] for the [`froglight-network`](crate) crate.
///
/// Adds networking and multiplayer support.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugins;

impl PluginGroup for NetworkPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ResolverPlugin::default()).add(StatusPlugin)
    }
}

impl Plugin for NetworkPlugins {
    fn build(&self, app: &mut App) { <Self as PluginGroup>::build(Self).finish(app); }
}
