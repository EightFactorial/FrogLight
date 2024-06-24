use std::time::Duration;

use bevy_app::App;
use bevy_ecs::{entity::Entity, event::Event};
use froglight_protocol::packet::ServerStatus;

use crate::connection::ConnectionError;

pub(super) fn build(app: &mut App) {
    app.add_event::<NetworkErrorEvent>().add_event::<ServerStatusResponse>();
}

/// An event for server status responses.
///
/// Sent when a server responds to a status request.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct ServerStatusResponse {
    /// The entity that received the status.
    pub entity: Entity,
    /// The status of the server.
    pub status: ServerStatus,
    /// The ping to the server.
    pub ping: Duration,
}

/// An event for network errors.
///
/// Sent when a connection error occurs.
#[derive(Debug, Event)]
pub struct NetworkErrorEvent {
    /// The entity that had the error.
    pub entity: Entity,
    /// The error that occurred.
    pub error: ConnectionError,
}
