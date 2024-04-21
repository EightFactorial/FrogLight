use bevy_ecs::component::Component;
use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use super::{
    parts::{PacketPair, TaskPair},
    traits::{PacketChannelTrait, PacketTrait, TaskChannelTrait},
};
use crate::connection::{NetworkDirection, Serverbound};

pub(crate) fn new_channel<V: Version>() -> (PacketChannel<V>, TaskChannel<V>)
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    let (login_send, login_recv) = super::parts::new_pair::<V, Login>();
    let (config_send, config_recv) = super::parts::new_pair::<V, Configuration>();
    let (play_send, play_recv) = super::parts::new_pair::<V, Play>();

    (
        PacketChannel { login: login_send, config: config_send, play: play_send },
        TaskChannel { login: login_recv, config: config_recv, play: play_recv },
    )
}

/// A bundle of packet channels for a specific [`Version`].
#[derive(Debug, Clone, Component)]
pub struct PacketChannel<V: Version>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// The [`Login`] packet channel.
    pub login: PacketPair<V, Login>,
    /// The [`Configuration`] packet channel.
    pub config: PacketPair<V, Configuration>,
    /// The [`Play`] packet channel.
    pub play: PacketPair<V, Play>,
}

impl<V: Version> PacketChannelTrait<V> for PacketChannel<V>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    type TaskHalf = TaskChannel<V>;
    fn new() -> (Self, Self::TaskHalf) { new_channel() }
}

impl<V: Version> PacketTrait<V, Login> for PacketChannel<V>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn get_pair(&self) -> &PacketPair<V, Login> { &self.login }
}

impl<V: Version> PacketTrait<V, Configuration> for PacketChannel<V>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn get_pair(&self) -> &PacketPair<V, Configuration> { &self.config }
}

impl<V: Version> PacketTrait<V, Play> for PacketChannel<V>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn get_pair(&self) -> &PacketPair<V, Play> { &self.play }
}

/// A channel used for passing packets between bevy and the server.
#[derive(Debug, Clone)]
pub struct TaskChannel<V: Version>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// The [`Login`] task channel.
    pub login: TaskPair<V, Login>,
    /// The [`Configuration`] task channel.
    pub config: TaskPair<V, Configuration>,
    /// The [`Play`] task channel.
    pub play: TaskPair<V, Play>,
}

impl<V: Version> TaskChannelTrait<V> for TaskChannel<V>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn login(&self) -> &TaskPair<V, Login> { &self.login }
}
