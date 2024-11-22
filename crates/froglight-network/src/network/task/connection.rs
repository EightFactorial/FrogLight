use bevy_ecs::{component::Component, entity::Entity, event::Event};
use bevy_tasks::{block_on, poll_once, Task};
use compact_str::CompactString;
use froglight_protocol::traits::Version;

use super::ConnectionErrorEvent;
use crate::connection::ConnectionError;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) { app.add_event::<ConnectionClosedEvent>(); }

/// An in-progress connection to a server.
#[derive(Component)]
pub struct ConnectionTask {
    /// The address of the server that is connected.
    pub address: CompactString,
    /// The version id of the server.
    pub version: i32,

    task: Task<Result<(), ConnectionError>>,
}

impl ConnectionTask {
    /// Create a new [`ConnectionTask`] from a [`Task`].
    #[must_use]
    pub fn new<V: Version>(
        address: impl Into<CompactString>,
        task: Task<Result<(), ConnectionError>>,
    ) -> Self {
        Self { address: address.into(), version: V::ID, task }
    }

    /// Poll the connection to a server.
    ///
    /// Returns a `ConnectionClosedEvent` if the connection was closed,
    /// otherwise returns `None`.
    pub fn poll(
        &mut self,
        entity: Entity,
    ) -> Option<Result<ConnectionClosedEvent, ConnectionErrorEvent>> {
        if let Some(result) = block_on(poll_once(&mut self.task)) {
            match result {
                Ok(()) => Some(Ok(ConnectionClosedEvent { entity, address: self.address.clone() })),
                Err(error) => {
                    Some(Err(ConnectionErrorEvent { entity, address: self.address.clone(), error }))
                }
            }
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
