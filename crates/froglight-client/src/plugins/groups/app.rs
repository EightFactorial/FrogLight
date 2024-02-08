use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    transform::TransformPlugin,
    DefaultPlugins,
};
use froglight_interface::InterfacePlugin;
use froglight_loading::LoadingPlugin;
use froglight_resourcepack::ResourcePackPlugin;

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
/// - [`ResourcePackPlugin`]
/// - [`InterfacePlugin`]
/// - [`LoadingPlugin`] # Enabled by the `default-loading` feature
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        // Start with bevy's default plugins
        let mut group = PluginGroup::build(DefaultPlugins)
            // Disable bevy's TransformPlugin
            .disable::<TransformPlugin>();

        // Disable logging in release builds, unless the `logging` feature is enabled
        #[cfg(not(any(debug_assertions, feature = "logging")))]
        {
            group = group.disable::<bevy::log::LogPlugin>();
        }

        // Add Client plugins
        group = ClientPlugins::build_group(ClientPlugins, group);

        // Add App-specific plugins
        group = group.add(InterfacePlugin).add(ResourcePackPlugin::new());

        // Add LoadingPlugin if the `default-loading` feature is enabled
        #[cfg(feature = "default-loading")]
        {
            group = group.add(LoadingPlugin::default());
        }

        group
    }
}

#[test]
fn test_build() { AppPlugins::build(AppPlugins); }
