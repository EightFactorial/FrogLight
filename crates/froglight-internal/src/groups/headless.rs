use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    log::LogPlugin,
    MinimalPlugins as BevyMinimalPlugins,
};

use crate::{groups::basic::BasicPlugins, TASKPOOL_SETTINGS};

/// A [`PluginGroup`] for creating a basic headless application.
///
/// Contains all the plugins required to run a headless application.
///
/// [`MinimalPlugins`](BevyMinimalPlugins):
/// - [`TaskPoolPlugin`](bevy::core::TaskPoolPlugin)
/// - [`TypeRegistrationPlugin`](bevy::core::TypeRegistrationPlugin)
/// - [`FrameCountPlugin`](bevy::core::FrameCountPlugin)
/// - [`TimePlugin`](bevy::time::TimePlugin)
/// - [`ScheduleRunnerPlugin`](bevy::app::ScheduleRunnerPlugin)
/// - [`LogPlugin`]
///
/// And the following `FrogLight` plugins:
/// - [`BlockPlugin`](crate::prelude::plugins::BlockPlugin)
/// - [`NetworkPlugin`](crate::prelude::plugins::NetworkPlugin)
/// - [`ResolverPlugin`](crate::prelude::plugins::ResolverPlugin)
/// - [`UtilityPlugin`](crate::prelude::plugins::UtilityPlugin)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeadlessPlugins;

impl PluginGroup for HeadlessPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(BevyMinimalPlugins).add(LogPlugin::default());

        // Configure the TaskPoolPlugin
        builder = builder.set(TaskPoolPlugin { task_pool_options: TASKPOOL_SETTINGS });

        // Add BasicPlugins
        builder.add_group(BasicPlugins)
    }
}
