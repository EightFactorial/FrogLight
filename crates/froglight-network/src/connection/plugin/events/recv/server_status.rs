use bevy_app::App;
use bevy_ecs::event::Event;
use froglight_protocol::packet::ServerStatus;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.add_event::<ServerStatusEvent>(); }

/// An event that is sent when a server status response is received.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct ServerStatusEvent {
    /// The server status.
    pub status: ServerStatus,
}
