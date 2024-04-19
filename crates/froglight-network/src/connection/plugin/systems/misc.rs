use std::{fmt::Debug, marker::PhantomData};

use bevy_ecs::{bundle::Bundle, component::Component, schedule::SystemSet};
use froglight_protocol::{
    common::GameProfile,
    states::{Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use super::traits::handler::ConnectionHandler;
use crate::connection::{plugin::channel::task::ConnectionTask, NetworkDirection, Serverbound};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(super) struct ConnectionPreUpdateSet<V: Version>(PhantomData<V>);

impl<V: Version> Debug for ConnectionPreUpdateSet<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionPreUpdateSet<{:?}>", V::default())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(super) struct ConnectionPostUpdateSet<V: Version>(PhantomData<V>);

impl<V: Version> Debug for ConnectionPostUpdateSet<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionPostUpdateSet<{:?}>", V::default())
    }
}

/// A marker component for a connection.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ConnectionMarker<V: Version>(PhantomData<V>);

impl<V: Version> Debug for ConnectionMarker<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionMarker<{:?}>", V::default())
    }
}

/// A bundle containing [`Connection`] components.
#[derive(Bundle)]
pub struct ConnectionBundle<V: Version + ConnectionHandler>
where
    Serverbound: NetworkDirection<V, Handshaking>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Play>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
    Play: State<V>,
{
    /// The player's game profile.
    pub profile: GameProfile,
    /// The connection channel.
    pub channel: <V as ConnectionHandler>::Channel,
    /// The connection task.
    pub task: ConnectionTask,
    /// A marker for the connection.
    pub marker: ConnectionMarker<V>,
}

impl<V: Version + ConnectionHandler> ConnectionBundle<V>
where
    Serverbound: NetworkDirection<V, Handshaking>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Play>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
    Play: State<V>,
{
    /// Creates a new [`ConnectionBundle`].
    #[must_use]
    pub fn new(
        profile: GameProfile,
        channel: <V as ConnectionHandler>::Channel,
        task: ConnectionTask,
    ) -> Self {
        Self { profile, channel, task, marker: ConnectionMarker(PhantomData) }
    }
}
