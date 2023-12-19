use bevy::app::App;

mod net;
pub use net::{ConnectionEvent, PingResponse, StatusRequest, StatusResponse};

pub(super) fn setup(app: &mut App) { net::setup(app); }
