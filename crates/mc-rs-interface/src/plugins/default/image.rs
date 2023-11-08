use bevy::{
    prelude::{ImagePlugin as BevyImagePlugin, *},
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        let mut default_sampler = ImageSamplerDescriptor::nearest();

        // Set the address mode to repeat
        default_sampler.address_mode_u = ImageAddressMode::Repeat;
        default_sampler.address_mode_v = ImageAddressMode::Repeat;
        default_sampler.address_mode_w = ImageAddressMode::Repeat;

        app.add_plugins(BevyImagePlugin { default_sampler });
    }
}
