//! Traits used for versioning and serialization of data.

use std::fmt::Debug;

pub use froglight_common::Version;

/// A Protocol state
///
/// Different states have different packets.
pub trait State<V: Version>: 'static + Debug + Default + Copy + Eq + Send + Sync {
    /// Packets sent from the client to the server
    type ServerboundPacket: PacketEnum;
    /// Packets sent from the server to the client
    type ClientboundPacket: PacketEnum;
}

/// A collection of packets that can be sent or received.
pub trait PacketEnum: Debug + PartialEq + Send + Sync {}
