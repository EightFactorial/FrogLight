use bevy_app::App;
use bevy_ecs::{entity::Entity, event::Event};
use compact_str::CompactString;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.add_event::<RequestConnectionEvent>(); }

/// An event that creates a new server connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct RequestConnectionEvent {
    /// The entity that requested the server connection.
    pub entity: Entity,
    /// The server address.
    pub address: CompactString,
    /// The connection's [`Version ID`](crate::traits::Version::ID).
    pub version_id: i32,
}
