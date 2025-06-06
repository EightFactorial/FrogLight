#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use froglight_packet::{
    state::ValidState,
    v1_21_4::{
        handshake::HandshakePacket,
        status::{PingResultPacket, QueryPingPacket, QueryRequestPacket, QueryResponsePacket},
    },
};
use smol_str::ToSmolStr;

use crate::{connection::raw::RawPacketVersion, prelude::*};

impl<V: ValidState<Handshake>> ClientConnection<V, Handshake> {
    /// Send a [`HandshakePacket`] to the server to initiate a connection.
    ///
    /// # Errors
    /// Returns an error if the client is unable to send the packet.
    #[inline]
    pub async fn handshake<M: 'static>(
        &mut self,
        address: impl ToSmolStr,
        port: u16,
        intent: ConnectionIntent,
    ) -> Result<(), ConnectionError>
    where
        V::Serverbound: RawPacketVersion<V, M>,
        HandshakePacket: Into<V::Serverbound>,
    {
        self.write::<M>(HandshakePacket::new::<V>(address, port, intent)).await
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: ValidState<Status>> ClientConnection<V, Status> {
    /// Send a [`QueryRequestPacket`] to the server
    /// and wait for a [`ServerStatus`] response.
    ///
    /// # Errors
    /// Returns an error if the client is unable to send the request,
    /// read the response, or the server sends an invalid response.
    #[inline]
    pub async fn query_status<M: 'static>(&mut self) -> Result<ServerStatus, ConnectionError>
    where
        V::Serverbound: RawPacketVersion<V, M>,
        QueryRequestPacket: Into<V::Serverbound>,
        V::Clientbound: RawPacketVersion<V, M> + TryInto<QueryResponsePacket>,
        <V::Clientbound as TryInto<QueryResponsePacket>>::Error: core::fmt::Debug,
    {
        self.write::<M>(QueryRequestPacket).await?;
        match self.read::<M>().await?.try_into() {
            Ok(QueryResponsePacket { status }) => Ok(status),
            Err(err) => {
                Err(ConnectionError::ReadRawPacket(Box::new(UnexpectedPacketError::from(err))))
            }
        }
    }

    /// Send a [`QueryPingPacket`] to the server
    /// and wait for a [`PingResultPacket`] response.
    ///
    /// # Errors
    /// Returns an error if the client is unable to send the request,
    /// read the response, or the server sends an invalid response.
    #[inline]
    pub async fn query_ping<M: 'static>(&mut self, ping: u64) -> Result<u64, ConnectionError>
    where
        V::Serverbound: RawPacketVersion<V, M>,
        QueryPingPacket: Into<V::Serverbound>,
        V::Clientbound: RawPacketVersion<V, M> + TryInto<PingResultPacket>,
        <V::Clientbound as TryInto<PingResultPacket>>::Error: core::fmt::Debug,
    {
        self.write::<M>(QueryPingPacket { ping }).await?;
        match self.read::<M>().await?.try_into() {
            Ok(PingResultPacket { pong }) => Ok(pong),
            Err(err) => {
                Err(ConnectionError::ReadRawPacket(Box::new(UnexpectedPacketError::from(err))))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error, derive_more::From)]
#[error("Expected a different packet type, received: {received:?}")]
struct UnexpectedPacketError<T> {
    pub received: T,
}
