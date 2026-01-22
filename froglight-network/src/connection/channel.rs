//! TODO

use async_channel::{Receiver, Sender, TryRecvError, TrySendError};

/// A channel that (sends)[`Receiver`] and (receives)[`Sender`] values.
#[derive(Debug, Clone)]
pub struct Channel<T, U> {
    receiver: Receiver<T>,
    sender: Sender<U>,
}

impl<T, U> Channel<T, U> {
    /// Create a new [`Channel`] from the given [`Receiver`] and
    /// [`Sender`].
    #[inline]
    #[must_use]
    pub const fn new(receiver: Receiver<T>, sender: Sender<U>) -> Self { Self { receiver, sender } }

    /// Create a new pair of connected [`Channel`]s.
    ///
    /// See also: [`async_channel::bounded`] and [`async_channel::unbounded`].
    ///
    /// # Panics
    ///
    /// Panics if the provided buffer size is zero.
    #[inline]
    #[must_use]
    pub fn new_pair(buffer: Option<usize>) -> (Channel<T, U>, Channel<U, T>) {
        let (tx1, rx1);
        let (tx2, rx2);

        if let Some(buffer) = buffer {
            (tx1, rx1) = async_channel::bounded(buffer);
            (tx2, rx2) = async_channel::bounded(buffer);
        } else {
            (tx1, rx1) = async_channel::unbounded();
            (tx2, rx2) = async_channel::unbounded();
        }

        (Channel::new(rx1, tx2), Channel::new(rx2, tx1))
    }

    /// Split the [`Channel`] into its internal [`Receiver`] and
    /// [`Sender`].
    #[inline]
    #[must_use]
    pub fn into_split(self) -> (Receiver<T>, Sender<U>) { (self.receiver, self.sender) }

    /// Attempt to receive a value from the connection.
    ///
    /// # Errors
    ///
    /// Returns an [`error`](TryRecvError) if there are no values available to
    /// receive or if the connection has been closed.
    #[inline]
    pub fn recv(&self) -> Result<T, TryRecvError> { self.receiver.try_recv() }

    /// Attempt to send a value to the connection.
    ///
    /// # Errors
    ///
    /// Returns an [`error`](TrySendError) if the connection is full
    /// or if the connection has been closed.
    #[inline]
    pub fn send(&self, value: U) -> Result<(), TrySendError<U>> { self.sender.try_send(value) }
}
