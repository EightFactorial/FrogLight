use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    diagnostic::DiagnosticsPlugin,
    log::LogPlugin,
    MinimalPlugins as BevyMinimalPlugins,
};
use froglight_settings::SettingsPlugin;

use crate::groups::basic::BasicPlugins;

/// A [`PluginGroup`] for all plugins that can be used in a headless
/// application.
///
/// Contains:
/// - [`MinimalPlugins`](BevyMinimalPlugins)
/// - [`LogPlugin`]
/// - [`DiagnosticsPlugin`]
/// - [`SettingsPlugin`]
/// - [`BasicPlugins`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeadlessPlugins;

impl PluginGroup for HeadlessPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = BevyMinimalPlugins.build();
        builder = builder.add(LogPlugin::default()).add(DiagnosticsPlugin);

        // Add the SettingsPlugin with the AssetSource disabled
        builder = builder.add(SettingsPlugin::default().disable_source());

        // Add BasicPlugins
        builder = BasicPlugins::add(builder);

        builder
    }
}
