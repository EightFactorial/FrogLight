//! [`Connection`] and its components

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::{error::Error, marker::PhantomData, net::SocketAddr};

use froglight_packet::state::{Config, Handshake, Login, Play, State, Status, ValidState};

use super::{RawConnection, raw::RawPacketVersion};

/// A [`RawConnection`] that manages the state of the connection.
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub struct Connection<V: ValidState<S>, S: State, D: Direction<V, S>> {
    raw: Box<dyn RawConnection>,
    scratch: Vec<u8>,
    _phantom: PhantomData<(V, S, D)>,
}

/// An error that can occur when reading or writing packets.
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    /// An error that occurred while parsing a packet from bytes.
    #[error("Failed to create raw packet from bytes: {0}")]
    ReadRawPacket(Box<dyn Error>),
    /// An error that occurred while reading from the connection.
    #[error("Failed to read from the connection: {0}")]
    ReadRawConnection(Box<dyn Error>),

    /// An error that occurred while writing a packet as bytes.
    #[error("Failed to write raw packet as bytes: {0}")]
    WriteRawPacket(Box<dyn Error>),
    /// An error that occurred while writing to the connection.
    #[error("Failed to write to the connection: {0}")]
    WriteRawConnection(Box<dyn Error>),
}

// -------------------------------------------------------------------------------------------------

/// The direction of the [`RawConnection`].
pub trait Direction<V: ValidState<S>, S: State> {
    /// The type of packet received by the connection.
    type Recv: Send + Sync;
    /// The type of packet sent by the connection.
    type Send: Send + Sync;
}

/// A [`RawConnection`] from a [`Client`] to a [`Server`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client;
impl<V: ValidState<S>, S: State> Direction<V, S> for Client {
    type Recv = V::Clientbound;
    type Send = V::Serverbound;
}

/// A [`RawConnection`] from a [`Server`] to a [`Client`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Server;
impl<V: ValidState<S>, S: State> Direction<V, S> for Server {
    type Recv = V::Serverbound;
    type Send = V::Clientbound;
}

// -------------------------------------------------------------------------------------------------

impl<V: ValidState<S>, S: State, D: Direction<V, S>> Connection<V, S, D> {
    /// Read a raw type from the [`Connection`],
    /// regardless of the actual state.
    ///
    /// You should be using [`StateConnection::read_packet`] to read the
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

    /// Write a raw type into the [`Connection`],
    /// regardless of the actual state.
    ///
    /// You should be using [`StateConnection::write_packet`] to write the
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

    /// Manually set the [`State`] of the [`Connection`].
    ///
    /// # Warning
    /// It is the responsibility of the caller to ensure the
    /// connection is expecting the state to be changed.
    #[inline]
    #[must_use]
    pub fn transform<S2: State>(self) -> Connection<V, S2, D>
    where
        V: ValidState<S2>,
        D: Direction<V, S2>,
    {
        Connection { raw: self.raw, scratch: self.scratch, _phantom: PhantomData }
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: ValidState<Handshake>, D: Direction<V, Handshake>> Connection<V, Handshake, D> {
    /// Create a new [`Connection`] from a [`RawConnection`].
    pub fn from_raw<T: RawConnection + 'static>(raw: T) -> Self {
        Connection { raw: Box::new(raw), scratch: Vec::new(), _phantom: PhantomData }
    }

    /// Enter the [`Status`] state.
    ///
    /// Use this when a connection is requesting the server's status.
    #[inline]
    #[must_use]
    pub fn status(self) -> Connection<V, Status, D>
    where
        V: ValidState<Status>,
        D: Direction<V, Status>,
    {
        self.transform()
    }

    /// Enter the [`Login`] state.
    ///
    /// Use this when the client will join the server.
    #[inline]
    #[must_use]
    pub fn login(self) -> Connection<V, Login, D>
    where
        V: ValidState<Login>,
        D: Direction<V, Login>,
    {
        self.transform()
    }
}

impl<V: ValidState<Login>, D: Direction<V, Login>> Connection<V, Login, D> {
    /// Enter the [`Config`] state.
    ///
    /// Use this when the client is ready to be configured.
    #[inline]
    #[must_use]
    pub fn config(self) -> Connection<V, Config, D>
    where
        V: ValidState<Config>,
        D: Direction<V, Config>,
    {
        self.transform()
    }
}

impl<V: ValidState<Config>, D: Direction<V, Config>> Connection<V, Config, D> {
    /// Enter the [`Play`] state.
    ///
    /// Use this when the client has finished being configured.
    #[inline]
    #[must_use]
    pub fn play(self) -> Connection<V, Play, D>
    where
        V: ValidState<Play>,
        D: Direction<V, Play>,
    {
        self.transform()
    }
}

impl<V: ValidState<Play>, D: Direction<V, Play>> Connection<V, Play, D> {
    /// Enter the [`Config`] state.
    ///
    /// Use this when the client is being reconfigured.
    #[inline]
    #[must_use]
    pub fn config(self) -> Connection<V, Config, D>
    where
        V: ValidState<Config>,
        D: Direction<V, Config>,
    {
        self.transform()
    }
}
