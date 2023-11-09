use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
    DefaultPlugins as BevyDefaultPlugins,
};

use crate::configs::settings::{window_settings::WindowSettings, Settings};

mod window;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DefaultPlugin {
    window: WindowSettings,
}

impl From<&Settings> for DefaultPlugin {
    fn from(value: &Settings) -> Self {
        Self {
            window: value.window.clone(),
        }
    }
}

impl PluginGroup for DefaultPlugin {
    fn build(self) -> PluginGroupBuilder {
        // Use the default bevy plugins
        let mut plugins = BevyDefaultPlugins::build(BevyDefaultPlugins);

        // Set the image sampler to nearest and the address mode to repeat
        {
            let mut default_sampler = ImageSamplerDescriptor::nearest();

            default_sampler.address_mode_u = ImageAddressMode::Repeat;
            default_sampler.address_mode_v = ImageAddressMode::Repeat;
            default_sampler.address_mode_w = ImageAddressMode::Repeat;

            plugins = plugins.set(ImagePlugin { default_sampler });
        }

        // Set the window title, resolution, vsync, etc.
        {
            plugins = window::setup(self.window, plugins);
        }

        plugins.build()
    }
}
