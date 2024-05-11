use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetPlugin as BevyAssetPlugin,
    diagnostic::DiagnosticsPlugin,
    DefaultPlugins as BevyDefaultPlugins,
};
use froglight_settings::SettingsPlugin;

use crate::groups::{basic::BasicPlugins, graphics::GraphicalPlugins};

/// A [`PluginGroup`] for all plugins that are used in the [`FrogLight`](crate)
/// application.
///
/// Contains:
/// - [`DefaultPlugins`](BevyDefaultPlugins)
/// - [`DiagnosticsPlugin`]
/// - [`SettingsPlugin`]
/// - [`BasicPlugins`]
/// - [`GraphicalPlugins`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = BevyDefaultPlugins.build();
        builder = builder.add(DiagnosticsPlugin);

        // Add the SettingsPlugin before the BevyAssetPlugin
        builder = builder.add_before::<BevyAssetPlugin, _>(SettingsPlugin::default());

        // Add BasicPlugins and GraphicalPlugins
        builder = BasicPlugins::add(builder);
        builder = GraphicalPlugins::add(builder);

        builder
    }
}
