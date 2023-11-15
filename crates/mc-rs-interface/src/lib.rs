use bevy::prelude::*;
use configs::settings::Settings;
use plugins::{DefaultPlugin, RenderPlugin};
use traits::config::ConfigFile;

use mc_rs_core::CorePlugin;

pub mod configs;
pub mod interface;
pub mod plugins;
pub mod resourcepacks;
pub mod resources;
pub mod traits;
pub mod util;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        resourcepacks::register_assets(app);

        let settings = Settings::load();
        app.add_plugins((DefaultPlugin::from(&settings), CorePlugin, RenderPlugin));

        settings.insert_resources(app);
        app.insert_resource(settings);

        resources::setup(app);
        interface::setup(app);
        resourcepacks::init_assets(app);
    }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
