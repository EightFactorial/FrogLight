use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    DefaultPlugins as BevyDefaultPlugins,
};

use super::{BasicPlugins, TASKPOOL_SETTINGS};

/// The default FrogLight [`PluginGroup`].
///
/// Contains all the plugins required to create an application,
/// including the default Bevy plugins.
///
/// # Note
/// Plugins are only added if the appropriate bevy features are enabled.
///
/// This includes:
/// [`BevyDefaultPlugins`]:
/// - [`PanicHandlerPlugin`](bevy::app::PanicHandlerPlugin)
/// - [`LogPlugin`](bevy::log::LogPlugin)
/// - [`TaskPoolPlugin`](bevy::core::TaskPoolPlugin)
/// - [`TypeRegistrationPlugin`](bevy::core::TypeRegistrationPlugin)
/// - [`FrameCountPlugin`](bevy::core::FrameCountPlugin)
/// - [`TimePlugin`](bevy::time::TimePlugin)
/// - [`TransformPlugin`](bevy::transform::TransformPlugin)
/// - [`HierarchyPlugin`](bevy::hierarchy::HierarchyPlugin)
/// - [`DiagnosticsPlugin`](bevy::diagnostic::DiagnosticsPlugin)
/// - [`InputPlugin`](bevy::input::InputPlugin)
/// - [`ScheduleRunnerPlugin`](bevy::app::ScheduleRunnerPlugin)
/// - [`WindowPlugin`](bevy::window::WindowPlugin)
/// - [`AccessibilityPlugin`](bevy::a11y::AccessibilityPlugin)
/// - [`TerminalCtrlCHandlerPlugin`](bevy::app::TerminalCtrlCHandlerPlugin)
/// - [`AssetPlugin`](bevy::asset::AssetPlugin)
/// - [`ScenePlugin`](bevy::scene::ScenePlugin)
/// - [`WinitPlugin`](bevy::winit::WinitPlugin)
/// - [`RenderPlugin`](bevy::render::RenderPlugin)
/// - [`ImagePlugin`](bevy::render::texture::ImagePlugin)
/// - [`PipelinedRenderingPlugin`](bevy::render::pipelined_rendering::PipelinedRenderingPlugin)
/// - [`CorePipelinePlugin`](bevy::core_pipeline::CorePipelinePlugin)
/// - [`SpritePlugin`](bevy::sprite::SpritePlugin)
/// - [`TextPlugin`](bevy::text::TextPlugin)
/// - [`UiPlugin`](bevy::ui::UiPlugin)
/// - [`PbrPlugin`](bevy::pbr::PbrPlugin)
/// - [`GltfPlugin`](bevy::gltf::GltfPlugin)
/// - [`AudioPlugin`](bevy::audio::AudioPlugin)
/// - [`GilrsPlugin`](bevy::gilrs::GilrsPlugin)
/// - [`AnimationPlugin`](bevy::animation::AnimationPlugin)
/// - [`GizmoPlugin`](bevy::gizmos::GizmoPlugin)
/// - [`StatesPlugin`](bevy::state::app::StatesPlugin)
/// - [`DevToolsPlugin`](bevy::dev_tools::DevToolsPlugin)
/// - [`CiTestingPlugin`](bevy::dev_tools::ci_testing::CiTestingPlugin)
/// - [`DefaultPickingPlugins`](bevy::picking::DefaultPickingPlugins)
///
/// [`BasicPlugins`]:
/// - [`BlockPlugin`](crate::prelude::plugins::BlockPlugin)
/// - [`EntityPlugin`](crate::prelude::plugins::EntityPlugin)
/// - [`NetworkPlugin`](crate::prelude::plugins::NetworkPlugin)
/// - [`RegistryPlugin`](crate::prelude::plugins::RegistryPlugin)
/// - [`ResolverPlugin`](crate::prelude::plugins::ResolverPlugin)
/// - [`UtilityPlugin`](crate::prelude::plugins::UtilityPlugin)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(BevyDefaultPlugins);
        builder = builder.add_group(BasicPlugins);

        // Configure the `TaskPoolPlugin`
        builder.set(TaskPoolPlugin { task_pool_options: TASKPOOL_SETTINGS })
    }
}
