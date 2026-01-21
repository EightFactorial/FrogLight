//! TODO

use froglight_common::version::Version;

/// A [`Version`]'s associated packet types.
pub trait PacketVersion: Version {}

impl<V: Version> PacketVersion for V {}

// -------------------------------------------------------------------------------------------------

/// The state of a connection.
///
/// Provides associated packet types for the connection's state.
pub trait PacketState<V: PacketVersion> {
    /// The packet sent from the server to the client.
    type Clientbound: Clone;
    /// The packet sent from the client to the server.
    type Serverbound: Clone;
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
    type Recv;
    /// The packet sent.
    type Send;
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
