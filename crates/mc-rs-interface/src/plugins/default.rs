use bevy::{
    a11y::AccessibilityPlugin,
    app::PluginGroupBuilder,
    audio::AudioPlugin,
    core_pipeline::CorePipelinePlugin,
    gizmos::GizmoPlugin,
    gltf::GltfPlugin,
    input::InputPlugin,
    pbr::PbrPlugin,
    prelude::*,
    render::{pipelined_rendering::PipelinedRenderingPlugin, RenderPlugin},
    scene::ScenePlugin,
    sprite::SpritePlugin,
    text::TextPlugin,
    ui::UiPlugin,
    winit::WinitPlugin,
};

use super::{asset::AssetPlugin, image::ImagePlugin, window::WindowPlugin};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DefaultPlugin;

impl PluginGroup for DefaultPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(WindowPlugin)
            .add(AccessibilityPlugin)
            .add(AssetPlugin)
            .add(ScenePlugin)
            .add(WinitPlugin)
            .add(RenderPlugin::default())
            .add(ImagePlugin)
            .add(PipelinedRenderingPlugin)
            .add(CorePipelinePlugin)
            .add(SpritePlugin)
            .add(TextPlugin)
            .add(UiPlugin)
            .add(PbrPlugin::default())
            .add(GltfPlugin::default())
            .add(AudioPlugin::default())
            .add(GizmoPlugin)
    }
}
