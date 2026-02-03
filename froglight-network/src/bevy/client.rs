use core::error::Error;

use bevy_ecs::{component::Component, world::EntityRef};
use bevy_tasks::Task;

use crate::{
    bevy::NetworkVersion,
    connection::{ConnectionError, Runtime},
    event::{ClientboundEventEnum, ServerboundEventEnum},
};

/// The client-side end of a network connection.
///
/// Sends [`ServerboundEventEnum`]s to the server and receives
/// [`ClientboundEventEnum`]s from the server.
#[derive(Component)]
pub struct ClientConnection {
    sender: Box<SenderFn>,
    receiver: Box<ReceiverFn>,
    task: Task<Result<(), Box<dyn Error + Send + Sync>>>,
}

type SenderFn = dyn for<'a> Fn(ServerboundEventEnum, EntityRef<'a>) -> Result<(), ConnectionError>
    + Send
    + Sync;
type ReceiverFn = dyn for<'a> Fn(EntityRef<'a>) -> Result<Option<ClientboundEventEnum>, ConnectionError>
    + Send
    + Sync;

impl ClientConnection {
    /// Create a new [`ClientConnection`] using the given connection.
    #[inline]
    #[must_use]
    pub fn new<V: NetworkVersion, R: Runtime<C>, C: Send>(connection: C) -> Self {
        V::wrap_connection::<R, C>(connection)
    }

    /// Create a new [`ClientConnection`] from the given
    /// sender and receiver functions and [`Task`].
    ///
    /// This is typically used internally by
    /// [`NetworkVersion::wrap_connection`].
    #[inline]
    #[must_use]
    pub const fn new_from_parts(
        sender: Box<SenderFn>,
        receiver: Box<ReceiverFn>,
        task: Task<Result<(), Box<dyn Error + Send + Sync>>>,
    ) -> Self {
        Self { sender, receiver, task }
    }

    /// Send a [`ServerboundEventEnum`] to the server.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the event cannot be sent.
    #[inline]
    pub fn send(
        &self,
        event: ServerboundEventEnum,
        entity: EntityRef<'_>,
    ) -> Result<(), ConnectionError> {
        (self.sender)(event, entity)
    }

    /// Receive a [`ClientboundEventEnum`] from the server.
    ///
    /// Returns `None` if there are no events to receive.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if an event cannot be received.
    #[inline]
    pub fn receive(
        &mut self,
        entity: EntityRef<'_>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError> {
        (self.receiver)(entity)
    }

    /// Poll the connection [`Task`] for completion.
    ///
    /// Returns `None` if the task is still running.
    ///
    /// # Warning
    ///
    /// If the task has been completed, this component should be
    /// removed from the ECS to avoid polling it again!
    ///
    /// # Errors
    ///
    /// Returns an error if the task has encountered an error.
    pub fn poll_task(&mut self) -> Option<Result<(), Box<dyn Error + Send + Sync>>> {
        futures_lite::future::block_on(futures_lite::future::poll_once(&mut self.task))
    }
}
