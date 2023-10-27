use bevy::{
    prelude::{ImagePlugin as BevyImagePlugin, *},
    render::{render_resource::AddressMode, texture::ImageSampler},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        let mut default_sampler = ImageSampler::nearest_descriptor();
        default_sampler.address_mode_u = AddressMode::Repeat;
        default_sampler.address_mode_v = AddressMode::Repeat;
        default_sampler.address_mode_w = AddressMode::Repeat;

        app.add_plugins(BevyImagePlugin { default_sampler });
    }
}
