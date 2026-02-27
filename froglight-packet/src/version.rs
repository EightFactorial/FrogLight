//! TODO

use core::fmt::{Debug, Display};

#[cfg(feature = "facet")]
use facet::Facet;
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

// -------------------------------------------------------------------------------------------------

/// The state of a connection.
///
/// Provides associated packet types for the connection's state.
pub trait PacketState<V: PacketVersion> {
    /// The packet sent from the server to the client.
    #[cfg(not(feature = "facet"))]
    type Clientbound: Debug + Clone + Send + Sync;
    /// The packet sent from the server to the client.
    #[cfg(feature = "facet")]
    type Clientbound: Debug + Clone + Send + Sync + Facet<'static> + 'static;

    /// The packet sent from the client to the server.
    #[cfg(not(feature = "facet"))]
    type Serverbound: Debug + Clone + Send + Sync;
    /// The packet sent from the client to the server.
    #[cfg(feature = "facet")]
    type Serverbound: Debug + Clone + Send + Sync + Facet<'static> + 'static;

    /// Check if the given [`Serverbound`] packet causes a state transition.
    ///
    /// If so, it returns the state the connection transitions into.
    fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum>;
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

/// An enum containing all connection states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacketStateEnum {
    /// The [`Handshake`] state.
    Handshake,
    /// The [`Status`] state.
    Status,
    /// The [`Login`] state.
    Login,
    /// The [`Config`] state.
    Config,
    /// The [`Play`] state.
    Play,
}

impl Display for PacketStateEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PacketStateEnum::Handshake => write!(f, "Handshake"),
            PacketStateEnum::Status => write!(f, "Status"),
            PacketStateEnum::Login => write!(f, "Login"),
            PacketStateEnum::Config => write!(f, "Config"),
            PacketStateEnum::Play => write!(f, "Play"),
        }
    }
}

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
    #[cfg(not(feature = "facet"))]
    type Recv: Debug + Clone + Send + Sync + 'static;
    /// The packet received.
    #[cfg(feature = "facet")]
    type Recv: Debug + Clone + Send + Sync + Facet<'static> + 'static;

    /// The packet sent.
    #[cfg(not(feature = "facet"))]
    type Send: Debug + Clone + Send + Sync + 'static;
    /// The packet sent.
    #[cfg(feature = "facet")]
    type Send: Debug + Clone + Send + Sync + Facet<'static> + 'static;
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
pub enum VersionPacketBidirectional<V: PacketVersion> {
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
