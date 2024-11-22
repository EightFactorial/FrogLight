use std::sync::Arc;

use froglight_protocol::{
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};

use crate::connection::NetworkDirection;

/// A packet sent through a connection channel.
pub enum ChannelSendPacket<V: Version, D>
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
    /// A packet to send to the [`Handshake`] state.
    Handshake(Arc<<D as NetworkDirection<V, Handshake>>::Send>),
    /// A packet to send to the [`Status`] state.
    Status(Arc<<D as NetworkDirection<V, Status>>::Send>),
    /// A packet to send to the [`Login`] state.
    Login(Arc<<D as NetworkDirection<V, Login>>::Send>),
    /// A packet to send to the [`Configuration`] state.
    Config(Arc<<D as NetworkDirection<V, Configuration>>::Send>),
    /// A packet to send to the [`Play`] state.
    Play(Arc<<D as NetworkDirection<V, Play>>::Send>),
}

impl<V: Version, D> ChannelSendPacket<V, D>
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
    pub fn from_handshake(packet: impl Into<<D as NetworkDirection<V, Handshake>>::Send>) -> Self {
        Self::Handshake(Arc::new(packet.into()))
    }
    /// Send a [`Status`] packet.
    pub fn from_status(packet: impl Into<<D as NetworkDirection<V, Status>>::Send>) -> Self {
        Self::Status(Arc::new(packet.into()))
    }
    /// Send a [`Login`] packet.
    pub fn from_login(packet: impl Into<<D as NetworkDirection<V, Login>>::Send>) -> Self {
        Self::Login(Arc::new(packet.into()))
    }
    /// Send a [`Configuration`] packet.
    pub fn from_config(packet: impl Into<<D as NetworkDirection<V, Configuration>>::Send>) -> Self {
        Self::Config(Arc::new(packet.into()))
    }
    /// Send a [`Play`] packet.
    pub fn from_play(packet: impl Into<<D as NetworkDirection<V, Play>>::Send>) -> Self {
        Self::Play(Arc::new(packet.into()))
    }
}

/// A packet received from a connection channel.
pub enum ChannelRecvPacket<V: Version, D>
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
    /// A packet received from the [`Handshake`] state.
    Handshake(Arc<<D as NetworkDirection<V, Handshake>>::Recv>),
    /// A packet received from the [`Status`] state.
    Status(Arc<<D as NetworkDirection<V, Status>>::Recv>),
    /// A packet received from the [`Login`] state.
    Login(Arc<<D as NetworkDirection<V, Login>>::Recv>),
    /// A packet received from the [`Configuration`] state.
    Config(Arc<<D as NetworkDirection<V, Configuration>>::Recv>),
    /// A packet received from the [`Play`] state.
    Play(Arc<<D as NetworkDirection<V, Play>>::Recv>),
}

impl<V: Version, D> ChannelRecvPacket<V, D>
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
    pub fn from_handshake(packet: impl Into<<D as NetworkDirection<V, Handshake>>::Recv>) -> Self {
        Self::Handshake(Arc::new(packet.into()))
    }
    /// Send a [`Status`] packet.
    pub fn from_status(packet: impl Into<<D as NetworkDirection<V, Status>>::Recv>) -> Self {
        Self::Status(Arc::new(packet.into()))
    }
    /// Send a [`Login`] packet.
    pub fn from_login(packet: impl Into<<D as NetworkDirection<V, Login>>::Recv>) -> Self {
        Self::Login(Arc::new(packet.into()))
    }
    /// Send a [`Configuration`] packet.
    pub fn from_config(packet: impl Into<<D as NetworkDirection<V, Configuration>>::Recv>) -> Self {
        Self::Config(Arc::new(packet.into()))
    }
    /// Send a [`Play`] packet.
    pub fn from_play(packet: impl Into<<D as NetworkDirection<V, Play>>::Recv>) -> Self {
        Self::Play(Arc::new(packet.into()))
    }
}
