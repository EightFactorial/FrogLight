#![allow(dead_code)]

use async_trait::async_trait;
use flume::{Receiver, Sender};
use log::error;
use mc_rs_proto::{
    types::{enums::ConnectionIntent, GameProfile},
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, ConnectionError, State, Version,
};

use super::request::{PingResponse, StatusResponse};

/// A trait for handling connections to a server
///
/// Each version of the protocol has a different implementation of this trait
/// using the appropriate packets for that [Version].
#[async_trait]
pub trait NetworkHandle: Version + Send + Sync + 'static
where
    Handshake: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    /// Handle connections in the handshake state
    async fn handshake_handle(
        con: Connection<Self, Handshake>,
        intention: ConnectionIntent,
    ) -> Result<Connection<Self, Handshake>, ConnectionError>;

    /// Handle connections in the status state
    async fn status_handle(
        con: Connection<Self, Status>,
    ) -> Result<(StatusResponse, PingResponse), ConnectionError>;

    /// Handle connections in the login state
    async fn login_handle(
        con: Connection<Self, Login>,
    ) -> Result<(Connection<Self, Login>, GameProfile), ConnectionError>;

    /// Handle connections in the configuration state
    async fn configuration_handle(
        con: Connection<Self, Configuration>,
    ) -> Result<Connection<Self, Configuration>, ConnectionError>;

    /// Handle connections in the play state
    async fn play_handle(
        con: ConnectionEnum<Self>,
        tx: Sender<Result<ConnectionData<Self>, ConnectionError>>,
        rx: Receiver<ConnectionSend<Self>>,
    );
}

/// A connection to a server
///
/// This is a wrapper around a connection to a server that allows for sending and receiving packets
/// in either the configuration or play state.
#[derive(Debug)]
pub enum ConnectionEnum<V: Version>
where
    Play: State<V>,
    Configuration: State<V>,
{
    Configuration(Connection<V, Configuration>),
    Play(Connection<V, Play>),
}

impl<V: Version> ConnectionEnum<V>
where
    Play: State<V>,
    Configuration: State<V>,
{
    /// Receive a packet from the connection
    pub async fn receive_packet(&mut self) -> Result<ConnectionData<V>, ConnectionError> {
        match self {
            ConnectionEnum::Configuration(con) => {
                Ok(ConnectionData::Configuration(con.receive_packet().await?))
            }
            ConnectionEnum::Play(con) => Ok(ConnectionData::Play(con.receive_packet().await?)),
        }
    }

    /// Send a packet to the connection
    pub async fn send_packet(&mut self, packet: ConnectionSend<V>) -> Result<(), ConnectionError> {
        match self {
            ConnectionEnum::Configuration(con) => match packet {
                ConnectionSend::Configuration(packet) => con.send_packet(packet).await,
                _ => {
                    error!("Invalid packet for connection configuration state");
                    Ok(())
                }
            },
            ConnectionEnum::Play(con) => match packet {
                ConnectionSend::Play(packet) => con.send_packet(packet).await,
                _ => {
                    error!("Invalid packet for connection play state");
                    Ok(())
                }
            },
        }
    }

    /// Consumes the connection and returns a new one with the given state
    pub fn with_state(self, state: ConnectionState) -> Self {
        match state {
            ConnectionState::Configuration => match self {
                ConnectionEnum::Play(con) => ConnectionEnum::Configuration(con.into()),
                _ => self,
            },
            ConnectionState::Play => match self {
                ConnectionEnum::Configuration(con) => ConnectionEnum::Play(con.into()),
                _ => self,
            },
        }
    }
}

/// The data received from a connection
#[derive(Debug, Clone)]
pub enum ConnectionData<V: Version>
where
    Play: State<V>,
    Configuration: State<V>,
{
    Configuration(<Configuration as State<V>>::Clientbound),
    Play(<Play as State<V>>::Clientbound),
    NewState(ConnectionState),
    Closed,
}

/// The state of a connection
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    Configuration,
    Play,
}

/// The data to send to a connection
#[derive(Debug, Clone)]
pub enum ConnectionSend<V: Version>
where
    Play: State<V>,
    Configuration: State<V>,
{
    Configuration(<Configuration as State<V>>::Serverbound),
    Play(<Play as State<V>>::Serverbound),
}
