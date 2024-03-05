//! Traits for packets and structs that can be read and written.

/// A Protocol version
pub trait Version: 'static + Copy + Eq {
    /// The protocol version number
    const PROTOCOL_VERSION: i32;
}

/// A Protocol state
///
/// Different states have different packets.
pub trait State<V: Version>: 'static + Copy + Eq {
    /// Packets sent from the client to the server
    type ServerboundPacket: PacketEnum;
    /// Packets sent from the server to the client
    type ClientboundPacket: PacketEnum;
}

/// A collection of packets that can be sent or received.
pub trait PacketEnum: Send + Sync {}
