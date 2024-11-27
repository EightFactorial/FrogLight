use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    log::LogPlugin,
    HeadlessPlugins as BevyHeadlessPlugins,
};

use crate::{groups::basic::BasicPlugins, TASKPOOL_SETTINGS};

/// A [`PluginGroup`] for creating a basic headless application.
///
/// Contains all the plugins required to run a headless application.
///
/// Bevy's [`HeadlessPlugins`](BevyHeadlessPlugins):
/// - [`PanicHandlerPlugin`](bevy::app::PanicHandlerPlugin)
/// - [`LogPlugin`]
/// - [`TaskPoolPlugin`]
/// - [`TypeRegistrationPlugin`](bevy::core::TypeRegistrationPlugin)
/// - [`FrameCountPlugin`](bevy::core::FrameCountPlugin)
/// - [`TimePlugin`](bevy::time::TimePlugin)
/// - [`TransformPlugin`](bevy::transform::TransformPlugin)
/// - [`HierarchyPlugin`](bevy::hierarchy::HierarchyPlugin)
/// - [`DiagnosticsPlugin`](bevy::diagnostic::DiagnosticsPlugin)
/// - [`ScheduleRunnerPlugin`](bevy::app::ScheduleRunnerPlugin)
/// - [`TerminalCtrlCHandlerPlugin`](bevy::app::TerminalCtrlCHandlerPlugin)
/// - [`StatesPlugin`](bevy_state::app::StatesPlugin)
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
        builder = builder.add_group(BevyHeadlessPlugins).add(LogPlugin::default());
        builder = builder.add_group(BasicPlugins);

        // Configure the TaskPoolPlugin
        builder.set(TaskPoolPlugin { task_pool_options: TASKPOOL_SETTINGS })
    }
}
