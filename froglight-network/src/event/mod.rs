//! TODO

use froglight_packet::version::{Clientbound, PacketVersion, Serverbound, VersionPacket};

use crate::connection::ConnectionError;

pub mod enums;
pub use enums::{ClientboundEventEnum, ServerboundEventEnum};

/// A trait for defining event-to-packet and packet-to-event conversions for a
/// specific [`Version`](froglight_common::version::Version).
pub trait EventVersion: PacketVersion {
    /// Convert a [`ClientboundEventEnum`] into a [`VersionPacket`].
    ///
    /// Returns `None` if no packet corresponds to the given event.
    ///
    /// # Errors
    ///
    /// Errors if the event is not recognized or valid.
    fn client_event_to_packet(
        event: ClientboundEventEnum,
    ) -> Result<Option<VersionPacket<Self, Clientbound>>, ConnectionError>;

    /// Convert a [`VersionPacket`] into a [`ClientboundEventEnum`].
    ///
    /// Returns `None` if no event corresponds to the given packet.
    ///
    /// # Errors
    ///
    /// Errors if the packet is not recognized or valid.
    fn client_packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError>;

    /// Convert a [`ServerboundEventEnum`] into a [`VersionPacket`].
    ///
    /// Returns `None` if no packet corresponds to the given event.
    ///
    /// # Errors
    ///
    /// Errors if the event is not recognized or valid.
    fn server_event_to_packet(
        event: ServerboundEventEnum,
    ) -> Result<Option<VersionPacket<Self, Serverbound>>, ConnectionError>;

    /// Convert a [`VersionPacket`] into a [`ServerboundEventEnum`].
    ///
    /// Returns `None` if no event corresponds to the given packet.
    ///
    /// # Errors
    ///
    /// Errors if the packet is not recognized or valid.
    fn server_packet_to_event(
        packet: VersionPacket<Self, Serverbound>,
    ) -> Result<Option<ServerboundEventEnum>, ConnectionError>;
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;
