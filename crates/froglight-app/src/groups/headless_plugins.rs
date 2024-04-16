use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    diagnostic::DiagnosticsPlugin,
    log::LogPlugin,
    MinimalPlugins,
};

use crate::prelude::plugins::*;

/// A [`PluginGroup`] for all plugins that are used in the headless application.
///
/// ---
///
/// This includes bevy's [`MinimalPlugins`], [`LogPlugin`], and
/// [`DiagnosticsPlugin`], so no need to add them yourself!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeadlessPlugins;

impl PluginGroup for HeadlessPlugins {
    fn build(self) -> PluginGroupBuilder {
        MinimalPlugins::build(MinimalPlugins)
            .add(LogPlugin::default())
            .add(DiagnosticsPlugin)
            .add(CorePlugin)
            .add(EntityPlugin)
            .add(RegistryPlugin)
            .add(UtilityPlugin)
            .add(NetworkPlugins)
    }
}
