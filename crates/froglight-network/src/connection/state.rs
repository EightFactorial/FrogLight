//! [`StateConnection`] and its components

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::{error::Error, marker::PhantomData};

use froglight_packet::{
    packet::ValidState,
    state::{Config, Handshake, Login, Play, State, Status},
};

use super::{RawConnection, raw::RawPacketVersion};

/// A wrapper over a [`RawConnection`] that manages the state of the connection.
pub struct StateConnection<S: State, V: ValidState<S>, D: Direction<S, V>> {
    raw: Box<dyn RawConnection>,
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
pub trait Direction<S: State, V: ValidState<S>> {
    /// The type of packet received by the connection.
    type Recv: Send + Sync;
    /// The type of packet sent by the connection.
    type Send: Send + Sync;
}

/// A [`RawConnection`] from a [`Client`] to a [`Server`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client;
impl<S: State, V: ValidState<S>> Direction<S, V> for Client {
    type Recv = V::Clientbound;
    type Send = V::Serverbound;
}

/// A [`RawConnection`] from a [`Server`] to a [`Client`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Server;
impl<S: State, V: ValidState<S>> Direction<S, V> for Server {
    type Recv = V::Serverbound;
    type Send = V::Clientbound;
}

// -------------------------------------------------------------------------------------------------

impl<S: State, V: ValidState<S>, D: Direction<S, V>> StateConnection<S, V, D> {
    /// Read a raw type from the [`StateConnection`],
    /// regardless of the actual state.
    ///
    /// You should be using [`StateConnection::read_packet`] to read the
    /// correct packet type for the current state.
    ///
    /// # Warning
    /// It is the responsibility of the caller to ensure that
    /// the data received matches the expected packet type.
    pub async fn read_raw<T: RawPacketVersion<V, M>, M: 'static>(
        &mut self,
    ) -> Result<T, ConnectionError> {
        let bytes = self.raw.read_raw().await.map_err(ConnectionError::ReadRawConnection)?;
        let (packet, remainder) =
            T::read_packet(bytes).await.map_err(ConnectionError::ReadRawPacket)?;

        let packet_length = bytes.len() - remainder.len();
        self.raw.consume_raw(packet_length).await;

        Ok(packet)
    }

    /// Read a packet from the [`StateConnection`].
    ///
    /// # Errors
    /// Returns an error if the packet could not be parsed,
    /// or data could not be read from the connection.
    #[inline]
    pub async fn read_packet<M: 'static>(&mut self) -> Result<D::Recv, ConnectionError>
    where D::Recv: RawPacketVersion<V, M> {
        self.read_raw::<_, M>().await
    }

    /// Write a raw type into the [`StateConnection`],
    /// regardless of the actual state.
    ///
    /// You should be using [`StateConnection::write_packet`] to write the
    /// correct packet type for the current state.
    ///
    /// # Warning
    /// It is the responsibility of the caller to ensure that
    /// the connection is expecting the type of packet being written.
    pub async fn write_raw<T: RawPacketVersion<V, M>, M: 'static>(
        &mut self,
        packet: &T,
    ) -> Result<(), ConnectionError> {
        let bytes = packet.write_packet().await.map_err(ConnectionError::WriteRawPacket)?;
        self.raw.write_raw(&bytes).await.map_err(ConnectionError::WriteRawConnection)?;
        Ok(())
    }

    /// Write a packet to the [`StateConnection`].
    ///
    /// # Errors
    /// Returns an error if the packet could not be written,
    /// or data could not be written to the connection.
    #[inline]
    pub async fn write_packet<M: 'static>(
        &mut self,
        packet: impl Into<D::Send>,
    ) -> Result<(), ConnectionError>
    where
        D::Send: RawPacketVersion<V, M>,
    {
        self.write_raw::<_, M>(&packet.into()).await
    }

    /// Write a packet to the [`StateConnection`].
    ///
    /// # Errors
    /// Returns an error if the packet could not be written,
    /// or data could not be written to the connection.
    #[inline]
    pub async fn write_packet_ref<M: 'static>(
        &mut self,
        packet: &D::Send,
    ) -> Result<(), ConnectionError>
    where
        D::Send: RawPacketVersion<V, M>,
    {
        self.write_raw::<_, M>(packet).await
    }

    /// Manually set the [`State`] of the [`StateConnection`].
    ///
    /// # Warning
    /// It is the responsibility of the caller to ensure the
    /// connection is expecting the state to be changed.
    #[inline]
    #[must_use]
    pub fn transform<S2: State>(self) -> StateConnection<S2, V, D>
    where
        V: ValidState<S2>,
        D: Direction<S2, V>,
    {
        StateConnection { raw: self.raw, _phantom: PhantomData }
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: ValidState<Handshake>, D: Direction<Handshake, V>> StateConnection<Handshake, V, D> {
    /// Create a new [`StateConnection`] from a [`RawConnection`].
    pub fn from_raw<T: RawConnection + 'static>(raw: T) -> Self {
        StateConnection { raw: Box::new(raw), _phantom: PhantomData }
    }

    /// Enter the [`Status`] state.
    ///
    /// Use this when a connection is requesting the server's status.
    #[inline]
    #[must_use]
    pub fn status(self) -> StateConnection<Status, V, D>
    where
        V: ValidState<Status>,
        D: Direction<Status, V>,
    {
        self.transform()
    }

    /// Enter the [`Login`] state.
    ///
    /// Use this when the client will join the server.
    #[inline]
    #[must_use]
    pub fn login(self) -> StateConnection<Login, V, D>
    where
        V: ValidState<Login>,
        D: Direction<Login, V>,
    {
        self.transform()
    }
}

impl<V: ValidState<Login>, D: Direction<Login, V>> StateConnection<Login, V, D> {
    /// Enter the [`Config`] state.
    ///
    /// Use this when the client is ready to be configured.
    #[inline]
    #[must_use]
    pub fn config(self) -> StateConnection<Config, V, D>
    where
        V: ValidState<Config>,
        D: Direction<Config, V>,
    {
        self.transform()
    }
}

impl<V: ValidState<Config>, D: Direction<Config, V>> StateConnection<Config, V, D> {
    /// Enter the [`Play`] state.
    ///
    /// Use this when the client has finished being configured.
    #[inline]
    #[must_use]
    pub fn play(self) -> StateConnection<Play, V, D>
    where
        V: ValidState<Play>,
        D: Direction<Play, V>,
    {
        self.transform()
    }
}

impl<V: ValidState<Play>, D: Direction<Play, V>> StateConnection<Play, V, D> {
    /// Enter the [`Config`] state.
    ///
    /// Use this when the client is being reconfigured.
    #[inline]
    #[must_use]
    pub fn config(self) -> StateConnection<Config, V, D>
    where
        V: ValidState<Config>,
        D: Direction<Config, V>,
    {
        self.transform()
    }
}
