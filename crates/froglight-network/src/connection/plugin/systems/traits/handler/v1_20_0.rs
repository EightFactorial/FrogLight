//! A manual implementation of the [`HandleConnection`] trait for [`V1_20_0`],
//! since it does not have a [`Configuration`](super::Configuration) state.

use bevy_app::App;
use froglight_protocol::versions::v1_20_0::V1_20_0;

use super::HandleConnection;
use crate::connection::LegacyChannel;

impl HandleConnection for V1_20_0 {
    type Channel = LegacyChannel<Self>;

    fn add_systems(_app: &mut App) {}
}
