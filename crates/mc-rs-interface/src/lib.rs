use bevy::prelude::*;
use mc_rs_render::RenderPlugin;
use plugins::default::DefaultPlugin;

pub mod util;

mod plugins;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugin, RenderPlugin));

        // TODO: Module setup fns
    }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
