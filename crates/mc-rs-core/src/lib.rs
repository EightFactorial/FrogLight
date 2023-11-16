#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use bevy::prelude::*;
use bevy_rapier3d::plugin::RapierPhysicsPlugin;

// Re-export mc-rs-protocol
pub use mc_rs_protocol::{types::*, versions};

pub mod blocks;
pub mod components;
pub mod resources;
pub mod schedule;
pub mod world;

mod net_event;
pub use net_event::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Add Rapier physics plugins
        app.add_plugins(RapierPhysicsPlugin::<()>::default());

        #[cfg(feature = "debug_rapier")]
        {
            app.add_plugins(bevy_rapier3d::render::RapierDebugRenderPlugin::default());
        }

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
