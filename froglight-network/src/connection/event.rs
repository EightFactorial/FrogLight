//! TODO
#![allow(missing_docs, reason = "WIP")]

use async_channel::{Receiver, Sender, TryRecvError, TrySendError};
#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, entity::Entity, world::DeferredWorld};

/// An event sent from the server to the client.
pub enum ClientboundEvent {
    Placeholder,
}

/// An event sent from the client to the server.
pub enum ServerboundEvent {
    Placeholder,
}

// -------------------------------------------------------------------------------------------------

/// A connection that (sends)[`Receiver`] and (receives)[`Sender`] values.
#[derive(Debug, Clone)]
pub struct ConnectionChannel<T, U> {
    receiver: Receiver<T>,
    sender: Sender<U>,
}

impl<T, U> ConnectionChannel<T, U> {
    /// Create a new [`ConnectionChannel`] from the given [`Receiver`] and
    /// [`Sender`].
    #[inline]
    #[must_use]
    pub const fn new(receiver: Receiver<T>, sender: Sender<U>) -> Self { Self { receiver, sender } }

    /// Create a new pair of connected [`ConnectionChannel`]s.
    ///
    /// See also: [`async_channel::bounded`] and [`async_channel::unbounded`].
    ///
    /// # Panics
    ///
    /// Panics if the provided buffer size is zero.
    #[inline]
    #[must_use]
    pub fn new_pair(buffer: Option<usize>) -> (ConnectionChannel<T, U>, ConnectionChannel<U, T>) {
        let (tx1, rx1);
        let (tx2, rx2);

        if let Some(buffer) = buffer {
            (tx1, rx1) = async_channel::bounded(buffer);
            (tx2, rx2) = async_channel::bounded(buffer);
        } else {
            (tx1, rx1) = async_channel::unbounded();
            (tx2, rx2) = async_channel::unbounded();
        }

        (ConnectionChannel::new(rx1, tx2), ConnectionChannel::new(rx2, tx1))
    }

    /// Attempt to receive a value from the connection.
    ///
    /// # Errors
    ///
    /// Returns an [`error`](TryRecvError) if there are no values available to
    /// receive or if the connection has been closed.
    #[inline]
    pub fn recv(&self) -> Result<T, TryRecvError> { self.receiver.try_recv() }

    /// Get a reference to the internal [`Receiver`].
    #[inline]
    #[must_use]
    pub const fn receiver(&self) -> &Receiver<T> { &self.receiver }

    /// Attempt to send a value to the connection.
    ///
    /// # Errors
    ///
    /// Returns an [`error`](TrySendError) if the connection is full
    /// or if the connection has been closed.
    #[inline]
    pub fn send(&self, value: U) -> Result<(), TrySendError<U>> { self.sender.try_send(value) }

    /// Get a reference to the internal [`Sender`].
    #[inline]
    #[must_use]
    pub const fn sender(&self) -> &Sender<U> { &self.sender }
}

// -------------------------------------------------------------------------------------------------

/// A connection that [sends](ServerboundEvent) and [receives](ClientboundEvent)
/// events.
///
/// Uses the boxed functions to allow for connections to access the
/// [`World`](bevy_ecs::world::World) when sending and receiving events.
#[cfg(feature = "bevy")]
#[derive(Component)]
pub struct EventConnection {
    reader: Box<RecvFn>,
    writer: Box<SendFn>,
}

#[cfg(feature = "bevy")]
type RecvFn =
    dyn FnMut(Entity, &mut DeferredWorld) -> Result<ClientboundEvent, TryRecvError> + Send + Sync;
#[cfg(feature = "bevy")]
type SendFn = dyn FnMut(
        Entity,
        ServerboundEvent,
        &mut DeferredWorld,
    ) -> Result<(), TrySendError<ServerboundEvent>>
    + Send
    + Sync;

#[cfg(feature = "bevy")]
impl EventConnection {
    /// Create a new [`EventConnection`] from the given reader and writer
    /// functions.
    #[inline]
    #[must_use]
    pub fn new(reader: Box<RecvFn>, writer: Box<SendFn>) -> Self { Self { reader, writer } }

    /// Attempt to receive a [`ClientboundEvent`] from the connection.
    ///
    /// # Errors
    ///
    /// Returns an [`error`](TryRecvError) if there are no events to receive
    /// or if the connection has been closed.
    #[inline]
    pub fn recv(
        &mut self,
        entity: Entity,
        world: &mut DeferredWorld,
    ) -> Result<ClientboundEvent, TryRecvError> {
        (self.reader)(entity, world)
    }

    /// Attempt to send a [`ServerboundEvent`] to the connection.
    ///
    /// # Errors
    ///
    /// Returns an [`error`](TrySendError) if the connection is full
    /// or if the connection has been closed.
    #[inline]
    pub fn send(
        &mut self,
        entity: Entity,
        event: ServerboundEvent,
        world: &mut DeferredWorld,
    ) -> Result<(), TrySendError<ServerboundEvent>> {
        (self.writer)(entity, event, world)
    }
}
