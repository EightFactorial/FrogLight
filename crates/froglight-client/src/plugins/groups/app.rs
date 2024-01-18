use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins,
};

use crate::plugins::ClientPlugins;

/// A [`PluginGroup`] that includes both [`FrogLight`](crate) plugins and
/// [`bevy's`](bevy) [`DefaultPlugins`](bevy::DefaultPlugins).
///
/// This is the recommended [`PluginGroup`] for most use cases.
///
/// ----
///
/// This group also includes several [`Plugins`](bevy::app::Plugin) that are
/// not part of [`ClientPlugins`]:
/// - TODO
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        // Start with bevy's default plugins.
        let mut group = PluginGroup::build(DefaultPlugins);

        // Add FrogLight Client plugins.
        group = ClientPlugins::build_group(ClientPlugins, group);

        // TODO: Add App specific plugins.

        group
    }
}

#[test]
fn test_build() { AppPlugins::build(AppPlugins); }
