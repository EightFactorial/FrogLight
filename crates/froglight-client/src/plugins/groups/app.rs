use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins,
};
use froglight_gui::GuiPlugin;

use crate::plugins::ClientPlugins;

/// A [`PluginGroup`] that includes both [`FrogLight`](crate) plugins and
/// [`bevy's`](bevy) [`DefaultPlugins`](bevy::DefaultPlugins).
///
/// This is the recommended [`PluginGroup`] for most use cases.
///
/// ---
///
/// ### Note:
/// Bevy's [`LogPlugin`](bevy::log::LogPlugin) is disabled in release builds,
/// unless the `logging` feature is enabled.
///
/// ----
///
/// This [`PluginGroup`] includes several [`Plugins`](bevy::app::Plugin) that
/// are not part of [`ClientPlugins`]:
/// - [`GuiPlugin`]
/// - `LoadingPlugin` # Enabled by the `default-loading` feature
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        // Start with bevy's default plugins.
        let mut group = PluginGroup::build(DefaultPlugins);

        // Disable logging in release builds, unless the `logging` feature is enabled.
        #[cfg(not(any(debug_assertions, feature = "logging")))]
        {
            group = group.disable::<bevy::log::LogPlugin>();
        }

        // Add FrogLight Client plugins.
        group = ClientPlugins::build_group(ClientPlugins, group);

        // Add App-specific plugins.
        group.add(GuiPlugin)
    }
}

#[test]
fn test_build() { AppPlugins::build(AppPlugins); }
