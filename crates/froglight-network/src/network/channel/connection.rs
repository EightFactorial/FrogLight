use async_channel::{
    Receiver, Recv as RecvFut, Send as SendFut, Sender, TryRecvError, TrySendError,
};
use bevy_ecs::component::Component;
use froglight_protocol::{
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};

use super::{ChannelRecvPacket, ChannelSendPacket};
use crate::connection::{NetworkDirection, Serverbound};

/// Create a new channel for sending and receiving packets
/// between bevy and a connection.
#[must_use]
pub fn new<V: Version, D>() -> (BevyConnectionChannel<V, D>, AsyncConnectionChannel<V, D>)
where
    D: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    let (a_send, b_recv) = async_channel::unbounded();
    let (b_send, a_recv) = async_channel::unbounded();
    (
        BevyConnectionChannel { sender: b_send, receiver: b_recv },
        AsyncConnectionChannel { sender: a_send, receiver: a_recv },
    )
}

/// A channel for sending and receiving packets to and from a connection.
#[derive(Component)]
pub struct BevyConnectionChannel<V: Version, D = Serverbound>
where
    D: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    sender: Sender<ChannelSendPacket<V, D>>,
    receiver: Receiver<ChannelRecvPacket<V, D>>,
}

impl<V: Version, D> BevyConnectionChannel<V, D>
where
    D: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Send a [`Handshake`] packet.
    pub fn send_handshake(&self, packet: impl Into<<D as NetworkDirection<V, Handshake>>::Send>) {
        let _ = self.try_send(ChannelSendPacket::from_handshake(packet));
    }

    /// Send a [`Status`] packet.
    pub fn send_status(&self, packet: impl Into<<D as NetworkDirection<V, Status>>::Send>) {
        let _ = self.try_send(ChannelSendPacket::from_status(packet));
    }

    /// Send a [`Login`] packet.
    pub fn send_login(&self, packet: impl Into<<D as NetworkDirection<V, Login>>::Send>) {
        let _ = self.try_send(ChannelSendPacket::from_login(packet));
    }

    /// Send a [`Configuration`] packet.
    pub fn send_configuration(
        &self,
        packet: impl Into<<D as NetworkDirection<V, Configuration>>::Send>,
    ) {
        let _ = self.try_send(ChannelSendPacket::from_config(packet));
    }

    /// Send a [`Play`] packet.
    pub fn send_play(&self, packet: impl Into<<D as NetworkDirection<V, Play>>::Send>) {
        let _ = self.try_send(ChannelSendPacket::from_play(packet));
    }

    /// Send a packet to the connection through the channel.
    #[inline]
    #[expect(clippy::missing_errors_doc)]
    pub fn try_send(
        &self,
        packet: ChannelSendPacket<V, D>,
    ) -> Result<(), TrySendError<ChannelSendPacket<V, D>>> {
        self.sender.try_send(packet)
    }

    /// Receive a packet to the connection through the channel.
    #[inline]
    #[expect(clippy::missing_errors_doc)]
    pub fn try_recv(&self) -> Result<ChannelRecvPacket<V, D>, TryRecvError> {
        self.receiver.try_recv()
    }
}

/// A channel for sending and receiving packets to and from bevy.
pub struct AsyncConnectionChannel<V: Version, D>
where
    D: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    sender: Sender<ChannelRecvPacket<V, D>>,
    receiver: Receiver<ChannelSendPacket<V, D>>,
}

impl<V: Version, D> AsyncConnectionChannel<V, D>
where
    D: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Send a packet to bevy through the channel.
    #[inline]
    pub fn send(&self, packet: ChannelRecvPacket<V, D>) -> SendFut<'_, ChannelRecvPacket<V, D>> {
        self.sender.send(packet)
    }

    /// Receive a packet from bevy through the channel.
    #[inline]
    pub fn recv(&self) -> RecvFut<'_, ChannelSendPacket<V, D>> { self.receiver.recv() }
}
