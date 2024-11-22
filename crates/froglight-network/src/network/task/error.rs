use bevy_app::App;
use bevy_ecs::{entity::Entity, event::Event};
use compact_str::CompactString;

use crate::connection::ConnectionError;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.add_event::<ConnectionErrorEvent>(); }

/// An [`Event`] that is returned when a connection error occurs.
#[derive(Debug, Event)]
pub struct ConnectionErrorEvent {
    /// The entity the connection was associated with.
    pub entity: Entity,
    /// The address of the server.
    pub address: CompactString,
    /// The error that occurred.
    pub error: ConnectionError,
}
