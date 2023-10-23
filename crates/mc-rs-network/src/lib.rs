#![feature(future_join)]

use bevy::prelude::*;
use mc_rs_protocol::versions::v1_20_0::V1_20_0;

mod network;
pub use network::ConnectionEvent;
use network::Network;

mod handle;
pub mod request;
pub mod task;

pub mod v1_20_0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        request::setup(app);

        <V1_20_0 as Network>::register(app);
    }
}
