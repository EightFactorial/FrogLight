#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::fmt::Debug;

use froglight_packet::{
    common::{
        PlayerProfile,
        profile::{PlayerUsername, PlayerUuid},
    },
    state::ValidState,
    v1_21_4::{
        handshake::HandshakeC2SPacket,
        status::{
            PingResultS2CPacket, QueryPingC2SPacket, QueryRequestC2SPacket, QueryResponseS2CPacket,
        },
    },
    v1_21_6::login::LoginHelloC2SPacket,
};
use smol_str::ToSmolStr;

use crate::{connection::raw::RawPacketVersion, prelude::*};

impl<V: ValidState<Handshake>> ClientConnection<V, Handshake> {
    /// Send a [`HandshakeC2SPacket`] to the server to initiate a connection.
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
        HandshakeC2SPacket: Into<V::Serverbound>,
    {
        self.write::<M>(HandshakeC2SPacket::new::<V>(address, port, intent)).await
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: ValidState<Status>> ClientConnection<V, Status> {
    /// Send a [`QueryRequestC2SPacket`] to the server
    /// and wait for a [`ServerStatus`] response.
    ///
    /// # Errors
    /// Returns an error if the client is unable to send the request,
    /// read the response, or the server sends an invalid response.
    #[inline]
    pub async fn query_status<M: 'static>(&mut self) -> Result<ServerStatus, ConnectionError>
    where
        V::Serverbound: RawPacketVersion<V, M>,
        QueryRequestC2SPacket: Into<V::Serverbound>,
        V::Clientbound: RawPacketVersion<V, M> + TryInto<QueryResponseS2CPacket>,
        <V::Clientbound as TryInto<QueryResponseS2CPacket>>::Error: Debug,
    {
        self.write::<M>(QueryRequestC2SPacket).await?;
        match self.read::<M>().await?.try_into() {
            Ok(QueryResponseS2CPacket { metadata }) => Ok(metadata),
            Err(err) => {
                Err(ConnectionError::ReadRawPacket(Box::new(UnexpectedPacketError::from(err))))
            }
        }
    }

    /// Send a [`QueryPingC2SPacket`] to the server
    /// and wait for a [`PingResultS2CPacket`] response.
    ///
    /// # Errors
    /// Returns an error if the client is unable to send the request,
    /// read the response, or the server sends an invalid response.
    #[inline]
    pub async fn query_ping<M: 'static>(&mut self, timestamp: u64) -> Result<u64, ConnectionError>
    where
        V::Serverbound: RawPacketVersion<V, M>,
        QueryPingC2SPacket: Into<V::Serverbound>,
        V::Clientbound: RawPacketVersion<V, M> + TryInto<PingResultS2CPacket>,
        <V::Clientbound as TryInto<PingResultS2CPacket>>::Error: Debug,
    {
        self.write::<M>(QueryPingC2SPacket { timestamp }).await?;
        match self.read::<M>().await?.try_into() {
            Ok(PingResultS2CPacket { timestamp }) => Ok(timestamp),
            Err(err) => {
                Err(ConnectionError::ReadRawPacket(Box::new(UnexpectedPacketError::from(err))))
            }
        }
    }
}

#[derive(Debug, thiserror::Error, derive_more::From)]
#[error("Expected a different packet type, received: {received:?}")]
struct UnexpectedPacketError<T> {
    pub received: T,
}

// -------------------------------------------------------------------------------------------------

impl<V: ValidState<Login>> ClientConnection<V, Login> {
    /// Send a [`LoginHelloC2SPacket`] to the server to start the login process.
    ///
    /// # Errors
    /// Returns an error if the client is unable to send the packet.
    #[inline]
    pub async fn login<M: 'static>(
        &mut self,
        name: PlayerUsername,
        uuid: PlayerUuid,
    ) -> Result<(), ConnectionError>
    where
        V::Serverbound: RawPacketVersion<V, M>,
        LoginHelloC2SPacket: Into<V::Serverbound>,
    {
        self.write::<M>(LoginHelloC2SPacket { name, uuid }).await
    }

    /// Send a [`LoginHelloC2SPacket`] to the server to start the login process.
    ///
    /// # Errors
    /// Returns an error if the client is unable to send the packet.
    #[inline]
    pub async fn login_profile<M: 'static>(
        &mut self,
        profile: &PlayerProfile,
    ) -> Result<(), ConnectionError>
    where
        V::Serverbound: RawPacketVersion<V, M>,
        LoginHelloC2SPacket: Into<V::Serverbound>,
    {
        self.login(profile.username.clone(), profile.uuid).await
    }

    /// Handle the login process by reading packets from the server
    /// and passing them to the provided handler function.
    ///
    /// When the handler returns a [`PlayerProfile`] the login process is
    /// considered complete.
    ///
    /// # Errors
    /// Returns an error if the handler returns an error,
    /// or if the connection is closed unexpectedly.
    pub async fn login_handle<M: 'static, Err>(
        &mut self,
        mut handler: impl AsyncFnMut(&mut Self, V::Clientbound) -> Result<Option<PlayerProfile>, Err>,
    ) -> Result<PlayerProfile, Err>
    where
        V::Clientbound: RawPacketVersion<V, M>,
        Err: From<ConnectionError>,
    {
        loop {
            let packet = self.read::<M>().await?;
            match handler(self, packet).await {
                Ok(None) => {}
                Ok(Some(profile)) => return Ok(profile),
                Err(err) => return Err(err),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: ValidState<Config>> ClientConnection<V, Config> {
    /// Handle the configuration process by reading packets from the server
    /// and passing them to the provided handler function.
    ///
    /// The handler indicates that the configuration is complete
    /// by returning `Ok(true)`.
    ///
    /// # Errors
    /// Returns an error if the handler returns an error,
    /// or if the connection is closed unexpectedly.
    pub async fn config_handle<M: 'static, Err>(
        &mut self,
        mut handler: impl AsyncFnMut(&mut Self, V::Clientbound) -> Result<bool, Err>,
    ) -> Result<(), Err>
    where
        V::Clientbound: RawPacketVersion<V, M>,
        Err: From<ConnectionError>,
    {
        loop {
            let packet = self.read::<M>().await?;
            match handler(self, packet).await {
                Ok(false) => {}
                Ok(true) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
