#![feature(future_join)]

use bevy::prelude::*;
use mc_rs_protocol::versions::v1_20_0::V1_20_0;

mod network;
use network::Network;

mod handle;
mod v1_20_0;

pub mod task;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
struct NetworkingSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PreUpdate, NetworkingSet);

        <V1_20_0 as Network>::register(app);
    }
}

#[cfg(all(feature = "simd", feature = "simd_advanced"))]
compile_error!("Cannot enable both the `simd` and `simd_advanced` features at the same time.");
