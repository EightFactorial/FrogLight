use std::sync::Arc;

use async_channel::{Receiver, Sender, TryRecvError, TrySendError};
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, Serverbound};

/// A set  of channels for sending and receiving packets for each state.
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ConnectionChannel<V: Version, D = Serverbound>
where
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    D: NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
{
    /// Channels for the [`Login`] state.
    pub login: PacketChannel<V, Login, D>,
    /// Channels for the [`Configuration`] state.
    pub config: PacketChannel<V, Configuration, D>,
    /// Channels for the [`Play`] state.
    pub play: PacketChannel<V, Play, D>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct ConnectionTaskChannel<V: Version, D>
where
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    D: NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
{
    /// Channels for the [`Login`] state.
    pub(super) login: PacketTaskChannel<V, Login, D>,
    /// Channels for the [`Configuration`] state.
    pub(super) config: PacketTaskChannel<V, Configuration, D>,
    /// Channels for the [`Play`] state.
    pub(super) play: PacketTaskChannel<V, Play, D>,
}

impl<V: Version, D> ConnectionChannel<V, D>
where
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    D: NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
{
    /// Create a new set of channels.
    ///
    /// The [`ConnectionChannel`] should be used in bevy's ECS, while the
    /// [`ConnectionTaskChannel`] should be used in async tasks.
    #[must_use]
    pub(super) fn new() -> (ConnectionChannel<V, D>, ConnectionTaskChannel<V, D>) {
        let (login_send, login_recv) = PacketChannel::new();
        let (config_send, config_recv) = PacketChannel::new();
        let (play_send, play_recv) = PacketChannel::new();

        (
            ConnectionChannel { login: login_send, config: config_send, play: play_send },
            ConnectionTaskChannel { login: login_recv, config: config_recv, play: play_recv },
        )
    }
}

/// A pair of channels for sending and receiving packets.
#[derive(Debug)]
pub struct PacketChannel<V: Version, S: State<V>, D: NetworkDirection<V, S>> {
    /// A [`Sender`] for sending packets of type [`D::Send`].
    pub send: Sender<Arc<D::Send>>,
    /// A [`Receiver`] for receiving packets of type [`D::Recv`].
    pub recv: Receiver<Arc<D::Recv>>,
}

/// A pair of channels for sending and receiving packets.
#[derive(Debug)]
pub(super) struct PacketTaskChannel<V: Version, S: State<V>, D: NetworkDirection<V, S>> {
    /// A [`Sender`] for sending packets of type [`D::Recv`].
    pub(super) send: Sender<Arc<D::Recv>>,
    /// A [`Receiver`] for receiving packets of type [`D::Send`].
    pub(super) recv: Receiver<Arc<D::Send>>,
}

impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> PacketTaskChannel<V, S, D> {
    /// Send a packet through the channel.
    ///
    /// # Errors
    /// This will return an error if the channel is full or closed.
    pub(super) fn send(&self, packet: D::Recv) -> async_channel::Send<'_, Arc<D::Recv>> {
        self.send.send(Arc::new(packet))
    }

    /// Receive a packet from the channel.
    ///
    /// # Errors
    /// This will return an error if the channel is empty or closed.
    pub(super) fn recv(&self) -> async_channel::Recv<'_, Arc<D::Send>> { self.recv.recv() }
}

impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> PacketChannel<V, S, D> {
    /// Create a new set of channels.
    ///
    /// The [`PacketChannel`] should be used in creating a
    /// [`ConnectionChannel`],
    /// while the [`PacketTaskChannel`] should be used a
    /// [`ConnectionTaskChannel`].
    #[must_use]
    pub(super) fn new() -> (PacketChannel<V, S, D>, PacketTaskChannel<V, S, D>) {
        let (ecs_send, ecs_recv) = async_channel::unbounded();
        let (task_send, task_recv) = async_channel::unbounded();

        (
            PacketChannel { send: ecs_send, recv: task_recv },
            PacketTaskChannel { send: task_send, recv: ecs_recv },
        )
    }

    /// Send a packet through the channel.
    ///
    /// # Errors
    /// This will return an error if the channel is full or closed.
    pub fn send(&self, packet: impl Into<D::Send>) -> Result<(), TrySendError<Arc<D::Send>>> {
        self.send.try_send(Arc::new(packet.into()))
    }

    /// Receive a packet from the channel.
    ///
    /// # Errors
    /// This will return an error if the channel is empty or closed.
    pub fn recv(&self) -> Result<Arc<D::Recv>, TryRecvError> { self.recv.try_recv() }
}
