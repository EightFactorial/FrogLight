use std::sync::Arc;

use async_channel::{Receiver, Sender, TryRecvError, TrySendError};
use bevy_ecs::component::Component;
use froglight_protocol::{
    states::Play,
    traits::{State, Version},
};

use super::async_task::LegacyPacketChannel;
use crate::connection::{
    plugin::channel::traits::ChannelType, ConnectionError, NetworkDirection, Serverbound,
};

/// A channel for sending and receiving packets.
///
/// Almost identical to
/// [`ConnectionChannel`](crate::connection::ConnectionChannel), but
/// cannot send or receive
/// [`Configuration`](froglight_protocol::states::Configuration) packets.
///
/// This is used for [`Versions`](Version) that do not have a
/// [`Configuration`](froglight_protocol::states::Configuration) state.
#[derive(Clone, Component)]
pub struct LegacyChannel<V: Version>
where
    Serverbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    pub(crate) receiver: Receiver<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>>,
    pub(crate) sender: Sender<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>>,
    pub(crate) errors: Receiver<ConnectionError>,
}

impl<V: Version> ChannelType for LegacyChannel<V>
where
    Serverbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    type TaskHalf = LegacyPacketChannel<V>;
    fn new_pair() -> (Self, Self::TaskHalf) { Self::new_channel() }
}

#[allow(clippy::type_complexity)]
impl<V: Version> LegacyChannel<V>
where
    Serverbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    /// Creates a new channel pair.
    pub(crate) fn new_channel() -> (Self, LegacyPacketChannel<V>) {
        let (chan_send, pckt_recv) = async_channel::unbounded();
        let (pckt_send, chan_recv) = async_channel::unbounded();
        let (errors_send, errors_recv) = async_channel::unbounded();

        (
            Self { receiver: chan_recv, sender: chan_send, errors: errors_recv },
            LegacyPacketChannel { receiver: pckt_recv, sender: pckt_send, errors: errors_send },
        )
    }

    /// Receives an error from the server.
    ///
    /// # Errors
    /// Returns an error if the channel is empty or closed.
    pub fn recv_error(&self) -> Result<ConnectionError, TryRecvError> { self.errors.try_recv() }

    /// Sends a packet to the server.
    ///
    /// # Errors
    /// Returns an error if the channel is full or closed.
    pub fn send_packet(
        &self,
        packet: Arc<<Serverbound as NetworkDirection<V, Play>>::Send>,
    ) -> Result<(), TrySendError<Arc<<Serverbound as NetworkDirection<V, Play>>::Send>>> {
        self.sender.try_send(packet)
    }

    /// Receives a packet from the server.
    ///
    /// # Errors
    /// Returns an error if the channel is empty or closed.
    pub fn recv_packet(
        &self,
    ) -> Result<Arc<<Serverbound as NetworkDirection<V, Play>>::Recv>, TryRecvError> {
        self.receiver.try_recv()
    }
}
