use bevy::{
    prelude::*,
    render::{renderer::RenderDevice, settings::WgpuFeatures, RenderApp},
};

pub mod terrain;

pub(super) fn setup(app: &mut App) { terrain::setup(app); }

/// Checks if the render device supports the required features for bindless textures.
pub(crate) fn check_bindless_texture_support(app: &mut App) {
    let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };

    let render_device = render_app.world.resource::<RenderDevice>();

    if !render_device
        .features()
        .contains(WgpuFeatures::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING)
    {
        panic!(
            "Render device doesn't support feature `SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING`, which is required for texture binding arrays!"
        );
    }
}
