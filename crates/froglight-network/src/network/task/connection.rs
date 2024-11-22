use bevy_ecs::{component::Component, entity::Entity, event::Event};
use bevy_tasks::Task;
use compact_str::CompactString;
use froglight_protocol::traits::Version;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) { app.add_event::<ConnectionClosedEvent>(); }

/// An in-progress connection to a server.
#[derive(Component)]
pub struct ConnectionTask {
    /// The address of the server that is connected.
    pub address: CompactString,
    /// The version id of the server.
    pub version: i32,

    task: Task<()>,
}

impl ConnectionTask {
    /// Create a new [`ConnectionTask`] from a [`Task`].
    #[must_use]
    pub fn new<V: Version>(address: impl Into<CompactString>, task: Task<()>) -> Self {
        Self { address: address.into(), version: V::ID, task }
    }

    /// Poll the connection to a server.
    ///
    /// Returns a `ConnectionClosedEvent` if the connection was closed,
    /// otherwise returns `None`.
    pub fn poll(&mut self, entity: Entity) -> Option<ConnectionClosedEvent> {
        if self.task.is_finished() {
            Some(ConnectionClosedEvent { entity, address: self.address.clone() })
        } else {
            None
        }
    }
}

/// An [`Event`] that is returned when a connection is closed.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct ConnectionClosedEvent {
    /// The entity the connection was associated with.
    pub entity: Entity,
    /// The address of the server.
    pub address: CompactString,
}
