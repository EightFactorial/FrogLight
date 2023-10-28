use bevy::prelude::*;

mod world;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) { world::setup(app); }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
