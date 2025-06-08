//! [`ReadConnection`], [`WriteConnection`], and [`SplittableConnection`]

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::{marker::PhantomData, net::SocketAddr};

use async_trait::async_trait;
use froglight_packet::state::{State, ValidState};

use super::raw::RawPacketVersion;
use crate::{
    connection::{RawConnection, state::Direction},
    prelude::{Connection, ConnectionError},
};

/// A [`RawConnection`] that can be split into two halves.
#[async_trait]
pub trait SplittableConnection: RawConnection {
    /// Split the connection into two separate halves.
    async fn split(&mut self) -> (Box<dyn CombinableConnection>, Box<dyn CombinableConnection>);
}

/// A [`RawConnection`] that can be recombined with another connection.
#[async_trait]
pub trait CombinableConnection: RawConnection {
    /// Recombine two connections into one.
    async fn recombine(
        &mut self,
        other: &mut dyn CombinableConnection,
    ) -> Box<dyn SplittableConnection>;
}

// -------------------------------------------------------------------------------------------------

/// A [`RawConnection`] that can be read from.
pub struct ReadConnection<V: ValidState<S>, S: State, D: Direction<V, S>> {
    pub(super) raw: Box<dyn CombinableConnection>,
    scratch: Vec<u8>,
    _phantom: PhantomData<(V, S, D)>,
}

impl<V: ValidState<S>, S: State, D: Direction<V, S>> ReadConnection<V, S, D> {
    /// Create a new [`ReadConnection`] from a [`RawConnection`].
    #[inline]
    #[must_use]
    pub fn from_raw<T: CombinableConnection + 'static>(raw: T) -> Self {
        Self::from_raw_box(Box::new(raw))
    }

    /// Create a new [`ReadConnection`] from a boxed [`RawConnection`].
    #[must_use]
    pub const fn from_raw_box(raw: Box<dyn CombinableConnection>) -> Self {
        ReadConnection { raw, scratch: Vec::new(), _phantom: PhantomData }
    }

    /// Read a raw type from the [`Connection`],
    /// regardless of the actual state.
    ///
    /// You should be using [`Connection::read`] to read the
    /// correct packet type for the current state.
    ///
    /// # Warning
    /// It is the responsibility of the caller to ensure that
    /// the data received matches the expected packet type.
    ///
    /// # Errors
    /// Returns an error if the type could not be read,
    /// or data could not be read from the connection.
    pub async fn read_raw<T: RawPacketVersion<V, M>, M: 'static>(
        &mut self,
    ) -> Result<T, ConnectionError> {
        T::read_packet(self.raw.as_mut(), &mut self.scratch).await
    }

    /// Read a packet from the [`Connection`].
    ///
    /// # Errors
    /// Returns an error if the packet could not be parsed,
    /// or data could not be read from the connection.
    #[inline]
    pub async fn read<M: 'static>(&mut self) -> Result<D::Recv, ConnectionError>
    where D::Recv: RawPacketVersion<V, M> {
        self.read_raw::<_, M>().await
    }

    /// Get the peer address of the [`Connection`].
    ///
    /// # Errors
    /// Returns an error if the peer address could not be retrieved.
    #[inline]
    pub async fn peer_addr(&self) -> Result<SocketAddr, ConnectionError> {
        self.raw.peer_addr().await
    }

    /// Recombine a [`ReadConnection`] with a [`WriteConnection`]
    /// to form a full [`Connection`].
    #[must_use]
    pub async fn recombine(mut self, mut write: WriteConnection<V, S, D>) -> Connection<V, S, D> {
        Connection::from_raw_box(self.raw.recombine(write.raw.as_mut()).await)
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`RawConnection`] that can be written to.
pub struct WriteConnection<V: ValidState<S>, S: State, D: Direction<V, S>> {
    pub(super) raw: Box<dyn CombinableConnection>,
    scratch: Vec<u8>,
    _phantom: PhantomData<(V, S, D)>,
}

impl<V: ValidState<S>, S: State, D: Direction<V, S>> WriteConnection<V, S, D> {
    /// Create a new [`WriteConnection`] from a [`RawConnection`].
    #[inline]
    #[must_use]
    pub fn from_raw<T: CombinableConnection + 'static>(raw: T) -> Self {
        Self::from_raw_box(Box::new(raw))
    }

    /// Create a new [`WriteConnection`] from a boxed [`RawConnection`].
    #[must_use]
    pub const fn from_raw_box(raw: Box<dyn CombinableConnection>) -> Self {
        WriteConnection { raw, scratch: Vec::new(), _phantom: PhantomData }
    }

    /// Write a raw type into the [`Connection`],
    /// regardless of the actual state.
    ///
    /// You should be using [`Connection::write`] to write the
    /// correct packet type for the current state.
    ///
    /// # Warning
    /// It is the responsibility of the caller to ensure that
    /// the connection is expecting the type of packet being written.
    ///
    /// # Errors
    /// Returns an error if the type could not be written,
    /// or data could not be written to the connection.
    #[inline]
    pub async fn write_raw<T: RawPacketVersion<V, M>, M: 'static>(
        &mut self,
        packet: &T,
    ) -> Result<(), ConnectionError> {
        T::write_packet(packet, self.raw.as_mut(), &mut self.scratch).await
    }

    /// Write a packet to the [`Connection`].
    ///
    /// # Errors
    /// Returns an error if the packet could not be written,
    /// or data could not be written to the connection.
    #[inline]
    pub async fn write<M: 'static>(
        &mut self,
        packet: impl Into<D::Send>,
    ) -> Result<(), ConnectionError>
    where
        D::Send: RawPacketVersion<V, M>,
    {
        self.write_raw::<_, M>(&packet.into()).await
    }

    /// Write a packet to the [`Connection`].
    ///
    /// # Errors
    /// Returns an error if the packet could not be written,
    /// or data could not be written to the connection.
    #[inline]
    pub async fn write_ref<M: 'static>(&mut self, packet: &D::Send) -> Result<(), ConnectionError>
    where D::Send: RawPacketVersion<V, M> {
        self.write_raw::<_, M>(packet).await
    }

    /// Get the peer address of the [`Connection`].
    ///
    /// # Errors
    /// Returns an error if the peer address could not be retrieved.
    #[inline]
    pub async fn peer_addr(&self) -> Result<SocketAddr, ConnectionError> {
        self.raw.peer_addr().await
    }

    /// Recombine a [`ReadConnection`] with a [`WriteConnection`]
    /// to form a full [`Connection`].
    #[must_use]
    pub async fn recombine(mut self, mut read: ReadConnection<V, S, D>) -> Connection<V, S, D> {
        Connection::from_raw_box(read.raw.recombine(self.raw.as_mut()).await)
    }
}
