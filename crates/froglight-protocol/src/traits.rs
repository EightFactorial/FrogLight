//! Traits for packets and structs that can be read and written.

use std::{fmt::Debug, hash::Hash};

/// A Protocol version
#[cfg(not(feature = "reflect"))]
pub trait Version: 'static + Debug + Default + Copy + Eq {
    /// The protocol version number
    const PROTOCOL_VERSION: i32;
}

/// A Protocol version
#[cfg(feature = "reflect")]
pub trait Version:
    'static + Debug + Default + Copy + Eq + Hash + bevy_reflect::Reflect + bevy_reflect::TypePath
{
    /// The protocol version number
    const PROTOCOL_VERSION: i32;
}

/// A Protocol state
///
/// Different states have different packets.
pub trait State<V: Version>: 'static + Default + Copy + Eq {
    /// Packets sent from the client to the server
    type ServerboundPacket: PacketEnum;
    /// Packets sent from the server to the client
    type ClientboundPacket: PacketEnum;
}

/// A collection of packets that can be sent or received.
pub trait PacketEnum: Send + Sync {}
