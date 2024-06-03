use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    diagnostic::{DiagnosticsPlugin, EntityCountDiagnosticsPlugin},
    log::LogPlugin,
    MinimalPlugins as BevyMinimalPlugins,
};

use crate::{groups::basic::BasicPlugins, TASKPOOL_SETTINGS};

/// A [`PluginGroup`] for all plugins that can be used in a headless
/// application.
///
/// Contains:
/// - [`MinimalPlugins`](BevyMinimalPlugins)
/// - [`LogPlugin`]
/// - [`DiagnosticsPlugin`]
/// - [`EntityCountDiagnosticsPlugin`]
/// - [`BasicPlugins`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeadlessPlugins;

impl PluginGroup for HeadlessPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = BevyMinimalPlugins.build();
        builder = builder
            .add(LogPlugin::default())
            .add(DiagnosticsPlugin)
            .add(EntityCountDiagnosticsPlugin);

        // Configure the TaskPoolPlugin
        builder = builder.set(TaskPoolPlugin { task_pool_options: TASKPOOL_SETTINGS });

        // Add BasicPlugins
        builder = BasicPlugins::add(builder);

        builder
    }
}
