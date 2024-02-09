use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetPlugin,
    transform::TransformPlugin,
    DefaultPlugins,
};
use froglight_debug::DebugPlugin;
use froglight_interface::InterfacePlugin;
use froglight_loading::LoadingPlugin;
use froglight_resourcepack::ResourcePackPlugin;
use froglight_settings::SettingsPlugin;

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
/// - [`SettingsPlugin`]
/// - [`ResourcePackPlugin`]
/// - [`InterfacePlugin`]
/// - [`LoadingPlugin`] # Enabled by the `default-loading` feature
/// - [`DebugPlugin`]
/// - `WorldInspectorPlugin` # Enabled by the `inspector` feature
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
        group = group
            // Add SettingsPlugin before AssetPlugin
            .add_before::<AssetPlugin, SettingsPlugin>(SettingsPlugin::default())
            // Add plugins
            .add(InterfacePlugin)
            .add(ResourcePackPlugin::new());

        // Add LoadingPlugin if the `default-loading` feature is enabled
        #[cfg(feature = "default-loading")]
        {
            group = group.add(LoadingPlugin::default());
        }

        // Add WorldInspectorPlugin if the `inspector` feature is enabled
        #[cfg(feature = "inspector")]
        {
            group = group.add(DebugPlugin { inspector: true });
        }
        #[cfg(not(feature = "inspector"))]
        #[allow(clippy::default_constructed_unit_structs)]
        {
            group = group.add(DebugPlugin::default());
        }

        group
    }
}

#[test]
fn test_build() { AppPlugins::build(AppPlugins); }
