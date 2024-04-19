use std::sync::Arc;

use async_channel::{Receiver, RecvError, SendError, Sender};
use froglight_protocol::{
    states::{Configuration, Play},
    traits::{State, Version},
};

use super::component::{ChannelReceiver, ChannelSender, ConnectionChannel};
use crate::connection::{ConnectionError, NetworkDirection, Serverbound};

#[derive(Clone)]
pub struct PacketChannel<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(crate) receiver: PacketReceiver<V>,
    pub(crate) sender: PacketSender<V>,
    pub(crate) errors: Sender<ConnectionError>,
}

impl<V: Version> PacketChannel<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(crate) fn new_channel() -> (ConnectionChannel<V>, Self) { ConnectionChannel::new_channel() }

    /// Sends an error to bevy.
    pub(crate) async fn send_error(
        &self,
        error: ConnectionError,
    ) -> Result<(), SendError<ConnectionError>> {
        self.errors.send(error).await
    }

    /// Sends a configuration packet to bevy.
    pub(crate) async fn send_config(
        &self,
        packet: Arc<<Serverbound as NetworkDirection<V, Configuration>>::Recv>,
    ) -> Result<(), SendError<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Recv>>> {
        self.sender.config.send(packet).await
    }

    /// Sends a play packet to bevy.
    pub(crate) async fn send_play(
        &self,
        packet: Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>,
    ) -> Result<(), SendError<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>>> {
        self.sender.play.send(packet).await
    }

    /// Receives a configuration packet from bevy.
    pub(crate) async fn recv_config(
        &self,
    ) -> Result<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Send>, RecvError> {
        self.receiver.config.recv().await
    }

    /// Receives a play packet from bevy.
    pub(crate) async fn recv_play(
        &self,
    ) -> Result<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>, RecvError> {
        self.receiver.play.recv().await
    }
}

/// Receives packets from bevy and sends them to the server.
#[derive(Clone)]
pub(crate) struct PacketReceiver<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(super) config: Receiver<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Send>>,
    pub(super) play: Receiver<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>>,
}

impl<V: Version> PacketReceiver<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(crate) fn new_receiver() -> (ChannelSender<V>, Self) { ChannelSender::new_sender() }
}

/// Receives packets from the server and sends them to bevy.
#[derive(Clone)]
pub(crate) struct PacketSender<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(super) config: Sender<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Recv>>,
    pub(super) play: Sender<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>>,
}

impl<V: Version> PacketSender<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(crate) fn new_sender() -> (ChannelReceiver<V>, Self) { ChannelReceiver::new_receiver() }
}
