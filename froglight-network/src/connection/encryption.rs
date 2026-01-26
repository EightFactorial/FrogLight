use core::marker::PhantomData;

use crate::connection::Runtime;

/// An encrypted connection that uses a specific [`Runtime`].
#[derive(Clone)]
pub struct Encrypted<R: Runtime<C>, C> {
    connection: C,
    _phantom: PhantomData<R>,
}

impl<R: Runtime<C>, C> Encrypted<R, C> {
    /// Create a new [`Encrypted`] connection.
    ///
    /// Has encryption disabled by default.
    #[must_use]
    pub const fn new(connection: C) -> Self { Self { connection, _phantom: PhantomData } }

    /// Change the [`Runtime`] of this [`Encrypted`] connection.
    #[inline]
    #[must_use]
    pub fn with_runtime<R2: Runtime<C>>(self) -> Encrypted<R2, C> {
        Encrypted { connection: self.connection, _phantom: PhantomData }
    }

    /// Get a reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &C { &self.connection }

    /// Get a mutable reference to the underlying raw connection.
    #[inline]
    #[must_use]
    pub const fn as_raw_mut(&mut self) -> &mut C { &mut self.connection }
}
