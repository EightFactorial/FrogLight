use bevy_app::App;
use bevy_ecs::{entity::Entity, event::Event};
use compact_str::{CompactString, ToCompactString};
use froglight_protocol::traits::Version;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.add_event::<RequestStatusEvent>(); }

/// An event that creates a new server connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct RequestStatusEvent {
    /// The entity that requested the server connection.
    pub entity: Entity,
    /// The server address.
    pub address: CompactString,
    /// The connection's [`Version ID`](crate::traits::Version::ID).
    pub version_id: i32,
}

impl RequestStatusEvent {
    /// Creates a new [`RequestStatusEvent`] using a [`Version::ID`](Version).
    pub fn new<V: Version>(entity: Entity, address: &(impl AsRef<str> + ?Sized)) -> Self {
        Self { entity, address: address.as_ref().to_compact_string(), version_id: V::ID }
    }

    /// Creates a new [`RequestStatusEvent`] with a manual version ID.
    #[must_use]
    pub fn new_manual(entity: Entity, address: &str, version_id: i32) -> Self {
        Self { entity, address: address.to_compact_string(), version_id }
    }
}
