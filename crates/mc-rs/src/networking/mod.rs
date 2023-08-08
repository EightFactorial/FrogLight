use std::fmt::Debug;

use bevy::prelude::*;
use mc_rs_proto::Version;

pub mod status_request;
use self::status_request::StatusRequest;

pub mod ping_request;
use self::ping_request::PingRequest;

pub mod v1_20_1;

/// Add networking systems to the app
pub(super) fn setup(app: &mut App) {
    status_request::setup(app);
    ping_request::setup(app);

    // TODO: Add networking systems
}

pub trait Networking: Version + Debug + Send + Sync + 'static {
    /// Register the networking systems to the app
    fn register(app: &mut App) {
        app.add_systems(Update, (Self::request_status, Self::request_ping));
    }

    /// Send status request
    fn request_status(events: EventReader<StatusRequest<Self>>);

    /// Send ping request
    fn request_ping(events: EventReader<PingRequest<Self>>);

    /// Handle connections in the handshake state
    fn handshake_handle();

    /// Handle connections in the status state
    fn status_handle();

    /// Handle connections in the login state
    fn login_handle();

    /// Handle connections in the configuration state
    fn configuration_handle();

    /// Handle connections in the play state
    fn play_handle();
}
