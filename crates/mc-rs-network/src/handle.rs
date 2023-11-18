use std::{future::join, sync::Arc};

use bevy::log::error;
use flume::{Receiver, Sender};
use futures_lite::Future;
use futures_locks::Mutex;
use mc_rs_core::{resources::player::username::Username, PingResponse, StatusResponse};
use mc_rs_protocol::{
    types::{enums::ConnectionIntent, GameProfile},
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, ConnectionError, State, Version,
};

/// A trait for handling connections to a server
///
/// Each version of the protocol has a different implementation of this trait
/// using the appropriate packets for that [Version].
pub trait NetworkHandle: Version + Send + Sync + 'static
where
    Handshake: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    /// Handle connections in the handshake state
    fn handshake_handle(
        conn: Connection<Self, Handshake>,
        intention: ConnectionIntent,
    ) -> impl Future<Output = Result<Connection<Self, Handshake>, ConnectionError>> + Send;

    /// Handle connections in the status state
    fn status_handle(
        conn: Connection<Self, Status>,
    ) -> impl Future<Output = Result<(StatusResponse, PingResponse), ConnectionError>> + Send;

    /// Handle connections in the login state
    fn login_handle(
        username: Username,
        conn: Connection<Self, Login>,
    ) -> impl Future<Output = Result<(Connection<Self, Login>, GameProfile), ConnectionError>> + Send;

    /// Handle connections in the play/configuration states
    fn packet_handle(
        conn: ConnectionEnum<Self>,
        tx: Sender<Result<ConnectionData<Self>, ConnectionError>>,
        rx: Receiver<ConnectionSend<Self>>,
    ) -> impl Future<Output = ()> + Send {
        async move {
            let conn = Arc::new(Mutex::new(conn));

            join!(
                // Receive packets from the connection and send them through the channel
                async {
                    loop {
                        if let Err(err) = tx
                            .send_async(conn.lock().await.receive_packet().await)
                            .await
                        {
                            error!("Failed to send packet through channel: {err}");
                            return;
                        }
                    }
                },
                // Receive packets from the channel and send them through the connection
                async {
                    loop {
                        match rx.recv_async().await {
                            Ok(data) => {
                                if let Err(e) = conn.lock().await.send_packet(data).await {
                                    error!("Failed to send packet to server: {:?}", e);
                                    return;
                                }
                            }
                            Err(err) => {
                                error!("Failed to receive packet from channel: {err}");
                                return;
                            }
                        }
                    }
                }
            )
            .await;
        }
    }
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
                    panic!("Invalid packet for connection configuration state");
                }
            },
            ConnectionEnum::Play(con) => match packet {
                ConnectionSend::Play(packet) => con.send_packet(packet).await,
                _ => {
                    panic!("Invalid packet for connection play state");
                }
            },
        }
    }

    /// Consumes the connection and returns a new one with the given state
    #[allow(dead_code)]
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
