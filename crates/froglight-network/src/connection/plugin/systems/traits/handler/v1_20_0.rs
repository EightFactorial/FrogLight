//! A manual implementation of the [`HandleConnection`] trait for [`V1_20_0`],
//! since it does not have the [`Configuration`](super::Configuration) state.

use bevy_app::App;
use froglight_protocol::versions::v1_20_0::V1_20_0;

use super::HandleConnection;

impl HandleConnection for V1_20_0 {
    fn add_systems(_app: &mut App) {}
}
