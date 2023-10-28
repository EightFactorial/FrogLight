use bevy::prelude::*;

// Re-export mc_rs_protocol
pub use mc_rs_protocol::{types::*, versions};

pub mod blocks;
pub mod components;
pub mod resources;
pub mod schedule;
pub mod world;

mod net_event;
pub use net_event::*;

mod plugins;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        plugins::setup(app);

        blocks::setup(app);
        components::setup(app);
        net_event::setup(app);
        resources::setup(app);
        schedule::setup(app);
        world::setup(app);
    }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
