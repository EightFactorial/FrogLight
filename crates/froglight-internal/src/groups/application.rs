use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetPlugin as BevyAssetPlugin,
    prelude::ImagePlugin,
    DefaultPlugins as BevyDefaultPlugins,
};
use froglight_asset::AssetSourcePlugin;

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
/// - [`AssetSourcePlugin`](crate::prelude::plugins::AssetSourcePlugin)
/// - [`AssetDefinitionPlugin`](crate::prelude::plugins::AssetDefinitionPlugin)
/// - [`AssetLoaderPlugin`](crate::prelude::plugins::AssetLoaderPlugin)
/// - [`AssetProcessorPlugin`](crate::prelude::plugins::AssetProcessorPlugin)
/// - [`AssetCatalogPlugin`](crate::prelude::plugins::AssetCatalogPlugin)
/// - [`LoadingScreenPlugin`](crate::prelude::plugins::LoadingScreenPlugin)
/// - [`EntropyPlugin`](bevy_rand::plugin::EntropyPlugin)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicationPlugins;

impl PluginGroup for ApplicationPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(BevyDefaultPlugins).set(ImagePlugin::default_linear());

        builder = builder.add_group(BasicPlugins).add_group(ClientPlugins);
        builder.add_before::<BevyAssetPlugin, _>(AssetSourcePlugin::default())
    }
}
