use bevy::prelude::*;
use mc_rs_protocol::versions::v1_20_0::V1_20_0;

use self::network::Network;

pub mod handle;
pub mod network;
pub mod request;
pub mod task;

pub mod v1_20_0;

/// Add networking systems to the app
pub(super) fn setup(app: &mut App) {
    request::setup(app);

    V1_20_0::register(app);
}
