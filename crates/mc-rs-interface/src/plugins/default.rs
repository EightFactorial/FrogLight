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

use crate::settings::{window_settings::WindowSettings, Settings};

use super::{asset::AssetPlugin, image::ImagePlugin, window::WindowPlugin};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DefaultPlugin {
    window: WindowSettings,
}

impl From<Settings> for DefaultPlugin {
    fn from(value: Settings) -> Self {
        Self {
            window: value.window,
        }
    }
}

impl PluginGroup for DefaultPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(WindowPlugin::from(self.window))
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
