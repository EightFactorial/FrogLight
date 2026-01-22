//! TODO

use core::fmt::{self, Debug, Display};

use crate::event::{ClientboundEvent, ServerboundEvent};

/// A connection that can send and receive event using a coroutine.
pub struct EventConnection<T> {
    sender: Box<SenderFn<T>>,
    receiver: Box<ReceiverFn<T>>,
}

type SenderFn<T> =
    dyn for<'a> FnMut(ServerboundEvent, &'a mut T) -> Result<(), ConnectionError> + Send + Sync;
type ReceiverFn<T> =
    dyn for<'a> FnMut(&'a mut T) -> Result<Option<ClientboundEvent>, ConnectionError> + Send + Sync;

/// An error that can occur while using a [`EventConnection`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ConnectionError {
    /// The connection has been closed.
    Closed,
}

impl<T> EventConnection<T> {
    /// Creates a new [`EventConnection`] from the given sender and receiver
    /// functions.
    #[inline]
    #[must_use]
    pub fn new<F1, F2>(sender: F1, receiver: F2) -> Self
    where
        F1: for<'a> FnMut(ServerboundEvent, &'a mut T) -> Result<(), ConnectionError>
            + Send
            + Sync
            + 'static,
        F2: for<'a> FnMut(&'a mut T) -> Result<Option<ClientboundEvent>, ConnectionError>
            + Send
            + Sync
            + 'static,
    {
        Self::new_boxed(Box::new(sender), Box::new(receiver))
    }

    /// Creates a new [`EventConnection`] from the given boxed sender and
    /// receiver functions.
    #[inline]
    #[must_use]
    pub const fn new_boxed(sender: Box<SenderFn<T>>, receiver: Box<ReceiverFn<T>>) -> Self {
        Self { sender, receiver }
    }

    /// Send a [`ServerboundEvent`] through the connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be sent.
    #[inline]
    pub fn send(&mut self, event: ServerboundEvent, data: &mut T) -> Result<(), ConnectionError> {
        (self.sender)(event, data)
    }

    /// Receive a [`ClientboundEvent`] from the connection.
    ///
    /// # Errors
    ///
    /// Returns an error if an event cannot be received.
    #[inline]
    pub fn recv(&mut self, data: &mut T) -> Result<Option<ClientboundEvent>, ConnectionError> {
        (self.receiver)(data)
    }
}

// -------------------------------------------------------------------------------------------------

impl Display for ConnectionError {
    #[allow(unreachable_patterns, reason = "Non-exhaustive")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::Closed => write!(f, "Connection closed"),
            _ => todo!(),
        }
    }
}
