use core::error::Error;

use bevy_ecs::{component::Component, world::DeferredWorld};
use bevy_tasks::Task;

#[cfg(feature = "futures-lite")]
use crate::connection::FuturesLite;
use crate::{
    bevy::NetworkVersion,
    connection::{ConnectionError, EventConnection, Runtime},
    event::{ClientboundEvent, ServerboundEvent},
};

/// The client-side end of a network connection.
///
/// Sends [`ServerboundEvent`]s to the server and receives
/// [`ClientboundEvent`]s from the server.
#[derive(Component)]
pub struct ClientConnection {
    connection: EventConnection<DeferredWorld<'static>>,
    task: Task<Result<(), Box<dyn Error + Send + Sync>>>,
}

impl ClientConnection {
    /// Create a new [`ClientConnection`] using the given [`Runtime`] and
    /// connection.
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
    #[must_use]
    pub const fn new_from(
        connection: EventConnection<DeferredWorld<'static>>,
        task: Task<Result<(), Box<dyn Error + Send + Sync>>>,
    ) -> Self {
        Self { connection, task }
    }

    /// Send a [`ServerboundEvent`] to the server.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if the event cannot be sent.
    #[inline]
    pub fn send(
        &mut self,
        event: ServerboundEvent,
        world: &mut DeferredWorld<'static>,
    ) -> Result<(), ConnectionError> {
        self.connection.send(event, world)
    }

    /// Receive a [`ClientboundEvent`] from the server.
    ///
    /// Returns `None` if there are no events to receive.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectionError`] if an event cannot be received.
    pub fn recv(
        &mut self,
        world: &mut DeferredWorld<'static>,
    ) -> Result<Option<ClientboundEvent>, ConnectionError> {
        self.connection.recv(world)
    }

    /// Poll the connection [`Task`] for completion.
    ///
    /// Returns `None` if the task is still running.
    ///
    /// If the task has been completed, this component should be
    /// removed from the ECS to avoid polling it again.
    ///
    /// # Errors
    ///
    /// Returns an error if the task has encountered an error.
    pub fn poll(&mut self) -> Option<Result<(), Box<dyn Error + Send + Sync>>> {
        futures_lite::future::block_on(futures_lite::future::poll_once(&mut self.task))
    }
}
