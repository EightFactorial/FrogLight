use std::fmt::Debug;

use bevy_log::error;
use froglight_protocol::{
    states::{Configuration, Play},
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

/// A connection that can be in either the [`Configuration`] or [`Play`] state.
#[derive(Debug, Clone)]
pub(crate) enum ConnectionEnum<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    Config(Connection<V, Configuration>),
    Play(Connection<V, Play>),
}

impl<V: Version> ConnectionEnum<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    #[must_use]
    pub(crate) fn from_config(connection: Connection<V, Configuration>) -> Self {
        ConnectionEnum::Config(connection)
    }

    #[must_use]
    pub(crate) fn from_play(connection: Connection<V, Play>) -> Self {
        ConnectionEnum::Play(connection)
    }

    /// Returns `true` if the connection is in the [`Configuration`] state.
    #[must_use]
    pub(crate) fn is_config(&self) -> bool { matches!(self, ConnectionEnum::Config(_)) }
    /// Returns `true` if the connection is in the [`Play`] state.
    #[must_use]
    pub(crate) fn is_play(&self) -> bool { matches!(self, ConnectionEnum::Play(_)) }

    /// Transforms into a [`Configuration`] [`Connection`].
    #[must_use]
    pub(crate) fn into_config(self) -> Self {
        match self {
            ConnectionEnum::Play(connection) => ConnectionEnum::Config(connection.configuration()),
            ConnectionEnum::Config(_) => self,
        }
    }

    /// Transforms into a [`Play`] [`Connection`].
    #[must_use]
    pub(crate) fn into_play(self) -> Self {
        match self {
            ConnectionEnum::Config(connection) => ConnectionEnum::Play(connection.play()),
            ConnectionEnum::Play(_) => self,
        }
    }

    /// Sends a [`Configuration`] packet to the server.
    pub(crate) async fn send_config(
        &mut self,
        packet: &<Serverbound as NetworkDirection<V, Configuration>>::Send,
    ) -> Result<(), Option<ConnectionError>> {
        match self {
            ConnectionEnum::Config(connection) => {
                connection.send_packet(packet).await.map_err(Some)
            }
            ConnectionEnum::Play(_) => {
                error!("Attempted to send a `Configuration` packet to a `Play` connection!");
                Err(None)
            }
        }
    }

    /// Sends a [`Play`] packet to the server.
    pub(crate) async fn send_play(
        &mut self,
        packet: &<Serverbound as NetworkDirection<V, Play>>::Send,
    ) -> Result<(), Option<ConnectionError>> {
        match self {
            ConnectionEnum::Play(connection) => connection.send_packet(packet).await.map_err(Some),
            ConnectionEnum::Config(_) => {
                error!("Attempted to send a `Play` packet to a `Configuration` connection!");
                Err(None)
            }
        }
    }

    /// Receives a [`Configuration`] packet from the server.
    pub(crate) async fn recv_config(
        &mut self,
    ) -> Result<Option<<Serverbound as NetworkDirection<V, Configuration>>::Recv>, ConnectionError>
    {
        match self {
            ConnectionEnum::Config(connection) => Ok(Some(connection.recv().await?)),
            ConnectionEnum::Play(_) => {
                error!("Attempted to receive a `Configuration` packet from a `Play` connection!");
                Ok(None)
            }
        }
    }

    /// Receives a [`Play`] packet from the server.
    pub(crate) async fn recv_play(
        &mut self,
    ) -> Result<Option<<Serverbound as NetworkDirection<V, Play>>::Recv>, ConnectionError> {
        match self {
            ConnectionEnum::Play(connection) => Ok(Some(connection.recv().await?)),
            ConnectionEnum::Config(_) => {
                error!("Attempted to receive a `Play` packet from a `Configuration` connection!");
                Ok(None)
            }
        }
    }
}
