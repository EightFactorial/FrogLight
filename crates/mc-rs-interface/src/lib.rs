use bevy::prelude::*;
use configs::settings::Settings;
use plugins::{DefaultPlugin, RenderPlugin};
use traits::config::ResourceConfig;

mod configs;
mod plugins;
mod resourcepacks;
mod resources;
mod traits;
mod util;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        resourcepacks::register_assets(app);

        let settings = Settings::setup(app);

        app.add_plugins((DefaultPlugin::from(settings.clone()), RenderPlugin))
            .insert_resource(settings);

        resourcepacks::init_assets(app);
        resources::setup(app);
    }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
