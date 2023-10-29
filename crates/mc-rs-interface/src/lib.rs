use bevy::prelude::*;
use mc_rs_render::RenderPlugin;
use plugins::default::DefaultPlugin;

pub mod keybinds;
pub mod settings;

mod plugins;
mod util;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        // Load settings and plugins
        let settings = settings::setup(app);
        app.add_plugins((DefaultPlugin::from(settings), RenderPlugin));

        // Setup submodules
        keybinds::setup(app);
    }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
