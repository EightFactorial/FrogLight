//! Traits for packets and structs that can be read and written.

use bevy_reflect::Reflect;

use crate::io::{FrogRead, FrogWrite};

/// A Protocol version
pub trait Version: 'static + Copy + Eq + Reflect {
    /// The protocol version number
    const PROTOCOL_VERSION: i32;
}

/// A Protocol state
///
/// Different states have different packets.
pub trait State<V: Version>: 'static + Copy + Eq + Reflect {
    /// Packets sent from the client to the server
    type ServerboundPacket: FrogRead + Packet;
    /// Packets sent from the server to the client
    type ClientboundPacket: FrogWrite + Packet;
}

/// A packet that can be sent or received.
pub trait Packet: Send + Sync + Reflect {}
