use core::error::Error;

use bevy_ecs::{component::Component, world::EntityRef};
use bevy_tasks::Task;

#[cfg(feature = "futures-lite")]
use crate::connection::FuturesLite;
use crate::{
    bevy::NetworkVersion,
    connection::{ConnectionError, Runtime},
    event::{ClientboundEvent, ServerboundEvent},
};

/// The client-side end of a network connection.
///
/// Sends [`ServerboundEvent`]s to the server and receives
/// [`ClientboundEvent`]s from the server.
#[derive(Component)]
pub struct ClientConnection {
    sender: Box<SenderFn>,
    receiver: Box<ReceiverFn>,
    task: Task<Result<(), Box<dyn Error + Send + Sync>>>,
}

type SenderFn =
    dyn for<'a> Fn(ServerboundEvent, EntityRef<'a>) -> Result<(), ConnectionError> + Send + Sync;
type ReceiverFn = dyn for<'a> Fn(EntityRef<'a>) -> Result<Option<ClientboundEvent>, ConnectionError>
    + Send
    + Sync;

impl ClientConnection {
    /// Create a new [`ClientConnection`] using the given connection.
    ///
    /// ## Note
    ///
    /// This method is only available when the `futures-lite` feature is
    /// enabled, as it relies on the [`FuturesLite`] [`Runtime`].
    #[inline]
    #[must_use]
    #[cfg(feature = "futures-lite")]
    pub fn new<V: NetworkVersion, C>(connection: C) -> Self
    where
        FuturesLite: Runtime<C>,
    {
        V::wrap_connection::<C>(connection)
    }

    /// Create a new [`ClientConnection`] from the given
    /// [`EventConnection`] and [`Task`].
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

    /// Send a [`ServerboundEvent`] to the server.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the event cannot be sent.
    #[inline]
    pub fn send(
        &self,
        event: ServerboundEvent,
        entity: EntityRef<'_>,
    ) -> Result<(), ConnectionError> {
        (self.sender)(event, entity)
    }

    /// Receive a [`ClientboundEvent`] from the server.
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
    ) -> Result<Option<ClientboundEvent>, ConnectionError> {
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
