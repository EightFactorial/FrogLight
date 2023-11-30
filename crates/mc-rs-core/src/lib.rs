#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bevy::prelude::*;

// Re-export mc-rs-protocol
pub use mc_rs_protocol::{types::*, versions};

pub mod blocks;
pub mod components;
pub mod events;
pub mod resources;
pub mod schedule;
pub mod sounds;
pub mod world;

/// A plugin that adds core systems and resources.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        blocks::setup(app);
        components::setup(app);
        events::setup(app);
        resources::setup(app);
        schedule::setup(app);
        sounds::setup(app);
        world::setup(app);
    }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
