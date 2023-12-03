use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "embedded/block_background.wgsl");
    app.add_plugins(UiMaterialPlugin::<BlockBackgroundMaterial>::default());
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct BlockBackgroundMaterial {
    #[uniform(0)]
    pub scale_x: f32,
    #[uniform(1)]
    pub scale_y: f32,
    #[texture(2)]
    #[sampler(3)]
    pub texture: Handle<Image>,
}

impl Default for BlockBackgroundMaterial {
    fn default() -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            texture: Default::default(),
        }
    }
}

impl UiMaterial for BlockBackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://mc_rs_gui/menus/shaders/embedded/block_background.wgsl".into()
    }
}
