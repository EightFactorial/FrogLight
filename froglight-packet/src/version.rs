//! TODO

use core::fmt::Debug;

use froglight_common::version::Version;

/// A [`Version`]'s associated packet types.
pub trait PacketVersion: Version {
    /// The [`Handshake`] state.
    type Handshake: PacketState<Self> + Debug + Copy + Send + Sync + 'static;
    /// The [`Status`] state.
    type Status: PacketState<Self> + Debug + Copy + Send + Sync + 'static;
    /// The [`Login`] state.
    type Login: PacketState<Self> + Debug + Copy + Send + Sync + 'static;
    /// The [`Config`] state.
    type Config: PacketState<Self> + Debug + Copy + Send + Sync + 'static;
    /// The [`Play`] state.
    type Play: PacketState<Self> + Debug + Copy + Send + Sync + 'static;
}

impl<V: Version> PacketVersion for V
where
    Handshake: PacketState<V>,
    Status: PacketState<V>,
    Login: PacketState<V>,
    Config: PacketState<V>,
    Play: PacketState<V>,
    Clientbound: PacketDirection<V, Handshake>
        + PacketDirection<V, Status>
        + PacketDirection<V, Login>
        + PacketDirection<V, Config>
        + PacketDirection<V, Play>,
    Serverbound: PacketDirection<V, Handshake>
        + PacketDirection<V, Status>
        + PacketDirection<V, Login>
        + PacketDirection<V, Config>
        + PacketDirection<V, Play>,
{
    type Config = Config;
    type Handshake = Handshake;
    type Login = Login;
    type Play = Play;
    type Status = Status;
}

// -------------------------------------------------------------------------------------------------

/// The state of a connection.
///
/// Provides associated packet types for the connection's state.
pub trait PacketState<V: PacketVersion> {
    /// The packet sent from the server to the client.
    type Clientbound: Debug + Clone + Send + Sync;
    /// The packet sent from the client to the server.
    type Serverbound: Debug + Clone + Send + Sync;
}

/// A connection in the `Handshake` state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handshake;

/// A connection in the `Status` state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status;

/// A connection in the `Login` state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Login;

/// A connection in the `Config` state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config;

/// A connection in the `Play` state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Play;

// -------------------------------------------------------------------------------------------------

/// The direction of a connection.
///
/// Provides associated packet types for the connection's direction.
pub trait PacketDirection<V: PacketVersion, S: PacketState<V>>
where
    Clientbound: PacketDirection<V, S>,
    Serverbound: PacketDirection<V, S>,
{
    /// The packet received.
    type Recv: Debug + Clone + Send + Sync;
    /// The packet sent.
    type Send: Debug + Clone + Send + Sync;
}

/// A connection from a server to a client.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clientbound;

impl<V: PacketVersion, S: PacketState<V>> PacketDirection<V, S> for Clientbound {
    type Recv = S::Serverbound;
    type Send = S::Clientbound;
}

/// A connection from a client to a server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Serverbound;

impl<V: PacketVersion, S: PacketState<V>> PacketDirection<V, S> for Serverbound {
    type Recv = S::Clientbound;
    type Send = S::Serverbound;
}

// -------------------------------------------------------------------------------------------------

/// An enum containing all [`Clientbound`] and [`Serverbound`]
/// packets for a specific [`Version`]
#[derive(Debug, Clone)]
pub enum VersionPacketDirectional<V: PacketVersion> {
    /// A packet sent from the server to the client.
    Clientbound(VersionPacket<V, Clientbound>),
    /// A packet sent from the client to the server.
    Serverbound(VersionPacket<V, Serverbound>),
}

/// An enum containing all packets for a specific [`Version`]
/// in a specific [`PacketDirection`].
#[derive(Debug, Clone)]
pub enum VersionPacket<V: PacketVersion, D>
where
    D: PacketDirection<V, V::Handshake>
        + PacketDirection<V, V::Status>
        + PacketDirection<V, V::Login>
        + PacketDirection<V, V::Config>
        + PacketDirection<V, V::Play>,
{
    /// A packet in the [`Handshake`] state.
    Handshake(<D as PacketDirection<V, V::Handshake>>::Send),
    /// A packet in the [`Status`] state.
    Status(<D as PacketDirection<V, V::Status>>::Send),
    /// A packet in the [`Login`] state.
    Login(<D as PacketDirection<V, V::Login>>::Send),
    /// A packet in the [`Config`] state.
    Config(<D as PacketDirection<V, V::Config>>::Send),
    /// A packet in the [`Play`] state.
    Play(<D as PacketDirection<V, V::Play>>::Send),
}
