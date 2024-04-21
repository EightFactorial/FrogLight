use bevy_ecs::component::Component;
use froglight_protocol::{
    states::{Login, Play},
    traits::{State, Version},
};

use super::{
    parts::{PacketPair, TaskPair},
    traits::{PacketChannelTrait, PacketTrait, TaskChannelTrait},
};
use crate::connection::{NetworkDirection, Serverbound};

/// Create a new pair of legacy packet and task channels.
pub(crate) fn new_legacy_channel<V: Version>() -> (LegacyPacketChannel<V>, LegacyTaskChannel<V>)
where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    let (login_send, login_recv) = super::parts::new_pair::<V, Login>();
    let (play_send, play_recv) = super::parts::new_pair::<V, Play>();

    (
        LegacyPacketChannel { login: login_send, play: play_send },
        LegacyTaskChannel { login: login_recv, play: play_recv },
    )
}

/// A bundle of packet channels for a specific [`Version`].
///
/// Does not have a [`Configuration`](froglight_protocol::states::Configuration)
/// state.
#[derive(Debug, Clone, Component)]
pub struct LegacyPacketChannel<V: Version>
where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    /// The [`Login`] packet channel.
    pub login: PacketPair<V, Login>,
    /// The [`Play`] packet channel.
    pub play: PacketPair<V, Play>,
}

impl<V: Version> PacketChannelTrait<V> for LegacyPacketChannel<V>
where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    type TaskHalf = LegacyTaskChannel<V>;
    fn new() -> (Self, Self::TaskHalf) { new_legacy_channel() }
}

impl<V: Version> PacketTrait<V, Login> for LegacyPacketChannel<V>
where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    fn get_pair(&self) -> &PacketPair<V, Login> { &self.login }
}

impl<V: Version> PacketTrait<V, Play> for LegacyPacketChannel<V>
where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    fn get_pair(&self) -> &PacketPair<V, Play> { &self.play }
}

/// A [`TaskChannel`](super::TaskChannel) that does not have
/// a [`Configuration`](froglight_protocol::states::Configuration) state.
#[derive(Debug, Clone)]
pub struct LegacyTaskChannel<V: Version>
where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    /// The [`Login`] task channel.
    pub login: TaskPair<V, Login>,
    /// The [`Play`] task channel.
    pub play: TaskPair<V, Play>,
}

impl<V: Version> TaskChannelTrait<V> for LegacyTaskChannel<V>
where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    fn login(&self) -> &TaskPair<V, Login> { &self.login }
}
