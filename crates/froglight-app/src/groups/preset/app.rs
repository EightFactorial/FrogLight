use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetPlugin as BevyAssetPlugin,
    core::TaskPoolPlugin,
    diagnostic::{DiagnosticsPlugin, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    DefaultPlugins as BevyDefaultPlugins,
};
use froglight_settings::SettingsPlugin;

use crate::{
    groups::{basic::BasicPlugins, graphics::GraphicalPlugins},
    TASKPOOL_SETTINGS,
};

/// A [`PluginGroup`] for all plugins that are used in the [`FrogLight`](crate)
/// application.
///
/// Contains:
/// - [`DefaultPlugins`](BevyDefaultPlugins)
/// - [`DiagnosticsPlugin`]
/// - [`EntityCountDiagnosticsPlugin`]
/// - [`FrameTimeDiagnosticsPlugin`]
/// - [`SettingsPlugin`]
/// - [`BasicPlugins`]
/// - [`GraphicalPlugins`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = BevyDefaultPlugins.build();
        builder = builder
            .add(DiagnosticsPlugin)
            .add(EntityCountDiagnosticsPlugin)
            .add(FrameTimeDiagnosticsPlugin);

        // Add the SettingsPlugin before the BevyAssetPlugin
        builder = builder.add_before::<BevyAssetPlugin, _>(SettingsPlugin::default());

        // Configure the TaskPoolPlugin
        builder = builder.set(TaskPoolPlugin { task_pool_options: TASKPOOL_SETTINGS });

        // Add BasicPlugins and GraphicalPlugins
        builder = BasicPlugins::add(builder);
        builder = GraphicalPlugins::add(builder);

        builder
    }
}
