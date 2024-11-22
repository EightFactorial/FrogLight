use std::time::Duration;

use bevy_app::App;
use bevy_ecs::{component::Component, entity::Entity, event::Event};
use bevy_tasks::{block_on, poll_once, Task};
use compact_str::CompactString;
use froglight_protocol::{packet::ServerStatus, traits::Version};

use super::ConnectionErrorEvent;
use crate::connection::ConnectionError;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.add_event::<StatusResponseEvent>(); }

/// An in-progress status request to a server.
#[derive(Component)]
pub struct StatusTask {
    /// The address of the server being polled.
    pub address: CompactString,
    /// The version id of the server.
    pub version: i32,

    task: Task<Result<(ServerStatus, Duration), ConnectionError>>,
}

impl StatusTask {
    /// Create a new [`StatusTask`] from a [`Task`].
    #[must_use]
    pub fn new<V: Version>(
        address: impl Into<CompactString>,
        task: Task<Result<(ServerStatus, Duration), ConnectionError>>,
    ) -> Self {
        Self { address: address.into(), version: V::ID, task }
    }

    /// Poll the status of a server.
    ///
    /// Returns a [`StatusResponseEvent`] if the server responded,
    /// otherwise returns `None`.
    pub fn poll(
        &mut self,
        entity: Entity,
    ) -> Option<Result<StatusResponseEvent, ConnectionErrorEvent>> {
        if let Some(result) = block_on(poll_once(&mut self.task)) {
            match result {
                Ok((status, ping)) => Some(Ok(StatusResponseEvent {
                    entity,
                    address: self.address.clone(),
                    status,
                    ping,
                })),
                Err(error) => {
                    Some(Err(ConnectionErrorEvent { entity, address: self.address.clone(), error }))
                }
            }
        } else {
            None
        }
    }
}

/// An [`Event`] that is returned when a server responds to a status request.
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct StatusResponseEvent {
    /// The entity the status request was associated with.
    pub entity: Entity,
    /// The address of the server.
    pub address: CompactString,
    /// The status of the server.
    pub status: ServerStatus,
    /// The ping to the server.
    pub ping: Duration,
}
