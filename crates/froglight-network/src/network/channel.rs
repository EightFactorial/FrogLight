#![allow(dead_code)]

use async_channel::{Receiver, Sender};
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
    pub send: Sender<D::Send>,
    /// A [`Receiver`] for receiving packets of type [`D::Recv`].
    pub recv: Receiver<D::Recv>,
}

/// A pair of channels for sending and receiving packets.
#[derive(Debug)]
pub(super) struct PacketTaskChannel<V: Version, S: State<V>, D: NetworkDirection<V, S>> {
    /// A [`Sender`] for sending packets of type [`D::Recv`].
    pub(super) send: Sender<D::Recv>,
    /// A [`Receiver`] for receiving packets of type [`D::Send`].
    pub(super) recv: Receiver<D::Send>,
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
}
