use std::fmt::Debug;

use bevy::prelude::*;

pub mod v1_20_1;

/// Add networking systems to the app
pub(super) fn setup(_app: &mut App) {
    // TODO: Add networking systems
}

pub trait Networking: Debug + Send + Sync + 'static {
    /// Register the networking systems to the app
    fn register();

    /// Send a status request to the server
    fn status_request();

    /// Send a ping request to the server
    fn ping_request();

    /// Handle connections in the handshake state
    fn handshake_handle();

    /// Handle connections in the status state
    fn status_handle();

    /// Handle connections in the login state
    fn login_handle();

    /// Handle connections in the play state
    fn play_handle();
}
