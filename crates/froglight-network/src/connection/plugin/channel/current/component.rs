use std::sync::Arc;

use async_channel::{Receiver, Sender, TryRecvError, TrySendError};
use bevy_ecs::component::Component;
use froglight_protocol::{
    states::{Configuration, Play},
    traits::{State, Version},
};

use super::async_task::{PacketChannel, PacketReceiver, PacketSender};
use crate::connection::{ConnectionError, NetworkDirection, Serverbound};

/// A channel for sending and receiving packets.
///
/// If your [`Version`] does not have a [`Configuration`] state,
/// use [`LegacyChannel`](crate::connection::LegacyChannel) instead.
#[derive(Clone, Component)]
pub struct ConnectionChannel<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(crate) receiver: ChannelReceiver<V>,
    pub(crate) sender: ChannelSender<V>,
    pub(crate) errors: Receiver<ConnectionError>,
}

#[allow(clippy::type_complexity)]
impl<V: Version> ConnectionChannel<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Creates a new channel.
    pub(crate) fn new_channel() -> (Self, PacketChannel<V>) {
        let (chan_recv, pckt_send) = ChannelReceiver::new_receiver();
        let (chan_send, pckt_recv) = ChannelSender::new_sender();
        let (errors_send, errors_recv) = async_channel::unbounded();

        (
            Self { receiver: chan_recv, sender: chan_send, errors: errors_recv },
            PacketChannel { receiver: pckt_recv, sender: pckt_send, errors: errors_send },
        )
    }

    /// Receives an error from the server.
    pub(crate) fn recv_error(&self) -> Result<ConnectionError, TryRecvError> {
        self.errors.try_recv()
    }

    /// Sends a configuration packet to the server.
    pub(crate) fn send_config(
        &self,
        packet: Arc<<Serverbound as NetworkDirection<V, Configuration>>::Send>,
    ) -> Result<(), TrySendError<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Send>>>
    {
        self.sender.config.try_send(packet)
    }

    /// Sends a play packet to the server.
    pub(crate) fn send_play(
        &self,
        packet: Arc<<Serverbound as NetworkDirection<V, Play>>::Send>,
    ) -> Result<(), TrySendError<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>>> {
        self.sender.play.try_send(packet)
    }

    /// Receives a configuration packet from the server.
    pub(crate) fn recv_config(
        &self,
    ) -> Result<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Recv>, TryRecvError> {
        self.receiver.config.try_recv()
    }

    /// Receives a play packet from the server.
    pub(crate) fn recv_play(
        &self,
    ) -> Result<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>, TryRecvError> {
        self.receiver.play.try_recv()
    }
}

#[derive(Clone)]
pub(crate) struct ChannelReceiver<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(super) config: Receiver<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Recv>>,
    pub(super) play: Receiver<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>>,
}

impl<V: Version> ChannelReceiver<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(crate) fn new_receiver() -> (Self, PacketSender<V>) {
        let (conf_send, conf_recv) = async_channel::unbounded();
        let (play_send, play_recv) = async_channel::unbounded();

        (
            Self { config: conf_recv, play: play_recv },
            PacketSender { config: conf_send, play: play_send },
        )
    }
}

#[derive(Clone)]
pub(crate) struct ChannelSender<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(super) config: Sender<Arc<<Serverbound as NetworkDirection<V, Configuration>>::Send>>,
    pub(super) play: Sender<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>>,
}

impl<V: Version> ChannelSender<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(crate) fn new_sender() -> (Self, PacketReceiver<V>) {
        let (conf_send, conf_recv) = async_channel::unbounded();
        let (play_send, play_recv) = async_channel::unbounded();

        (
            Self { config: conf_send, play: play_send },
            PacketReceiver { config: conf_recv, play: play_recv },
        )
    }
}
