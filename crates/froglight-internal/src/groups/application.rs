use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    DefaultPlugins as BevyDefaultPlugins,
};

use super::{BasicPlugins, ClientPlugins};

/// A [`PluginGroup`] for creating a custom client.
///
/// Contains all the plugins required to run a client application.
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
/// - [`WindowPlugin`](bevy::window::WindowPlugin)
/// - [`AccessibilityPlugin`](bevy::a11y::AccessibilityPlugin)
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
///
/// [`BasicPlugins`]:
/// - [`NetworkPlugin`](crate::prelude::plugins::NetworkPlugin)
/// - [`ResolverPlugin`](crate::prelude::plugins::ResolverPlugin)
/// - [`UtilityPlugin`](crate::prelude::plugins::UtilityPlugin)
/// - [`BlockPlugin`](crate::prelude::plugins::BlockPlugin)
///
/// [`ClientPlugins`]:
/// - [`AssetDefinitionPlugin`](crate::prelude::plugins::AssetDefinitionPlugin)
/// - [`AssetLoaderPlugin`](crate::prelude::plugins::AssetLoaderPlugin)
/// - [`AssetProcessorPlugin`](crate::prelude::plugins::AssetProcessorPlugin)
/// - [`AssetStoragePlugin`](crate::prelude::plugins::AssetStoragePlugin)
/// - [`LoadingScreenPlugin`](crate::prelude::plugins::LoadingScreenPlugin)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicationPlugins;

impl PluginGroup for ApplicationPlugins {
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>();
        builder.add_group(BevyDefaultPlugins).add_group(BasicPlugins).add_group(ClientPlugins)
    }
}
