//! TODO

use froglight_packet::version::{
    Clientbound, PacketVersion, Serverbound, VersionPacket, VersionPacketBidirectional,
};

use crate::event::EventVersion;

/// A translator for converting packets between two [`Version`]s.
#[expect(clippy::type_complexity, reason = "Function Traits")]
pub struct PacketTranslator<A: PacketVersion, B: PacketVersion> {
    clientbound_a_to_b:
        Box<dyn FnMut(VersionPacket<A, Clientbound>) -> Option<VersionPacket<B, Clientbound>>>,
    serverbound_a_to_b:
        Box<dyn FnMut(VersionPacket<A, Serverbound>) -> Option<VersionPacket<B, Serverbound>>>,
    clientbound_b_to_a:
        Box<dyn FnMut(VersionPacket<B, Clientbound>) -> Option<VersionPacket<A, Clientbound>>>,
    serverbound_b_to_a:
        Box<dyn FnMut(VersionPacket<B, Serverbound>) -> Option<VersionPacket<A, Serverbound>>>,
}

impl<A: EventVersion + PacketVersion, B: EventVersion + PacketVersion> Default
    for PacketTranslator<A, B>
{
    fn default() -> Self { Self::new() }
}

impl<A: PacketVersion, B: PacketVersion> PacketTranslator<A, B> {
    /// Create a new [`PacketTranslator`] that uses the [`EventVersion`] trait.
    #[must_use]
    pub fn new() -> Self
    where
        A: EventVersion,
        B: EventVersion,
    {
        Self::new_using(
            |packet| match A::client_packet_to_event(packet) {
                Ok(Some(event)) => match B::client_event_to_packet(event) {
                    Ok(Some(packet)) => Some(packet),
                    Ok(None) | Err(_) => None,
                },
                Ok(None) | Err(_) => None,
            },
            |packet| match A::server_packet_to_event(packet) {
                Ok(Some(event)) => match B::server_event_to_packet(event) {
                    Ok(Some(packet)) => Some(packet),
                    Ok(None) | Err(_) => None,
                },
                Ok(None) | Err(_) => None,
            },
            |packet| match B::client_packet_to_event(packet) {
                Ok(Some(event)) => match A::client_event_to_packet(event) {
                    Ok(Some(packet)) => Some(packet),
                    Ok(None) | Err(_) => None,
                },
                Ok(None) | Err(_) => None,
            },
            |packet| match B::server_packet_to_event(packet) {
                Ok(Some(event)) => match A::server_event_to_packet(event) {
                    Ok(Some(packet)) => Some(packet),
                    Ok(None) | Err(_) => None,
                },
                Ok(None) | Err(_) => None,
            },
        )
    }

    /// Create a new [`PacketTranslator`] using the provided functions.
    ///
    /// If you have already-boxed functions,
    /// use [`PacketTranslator::new_using_boxed`] instead.
    #[inline]
    #[must_use]
    pub fn new_using<
        F1: FnMut(VersionPacket<A, Clientbound>) -> Option<VersionPacket<B, Clientbound>> + 'static,
        F2: FnMut(VersionPacket<A, Serverbound>) -> Option<VersionPacket<B, Serverbound>> + 'static,
        F3: FnMut(VersionPacket<B, Clientbound>) -> Option<VersionPacket<A, Clientbound>> + 'static,
        F4: FnMut(VersionPacket<B, Serverbound>) -> Option<VersionPacket<A, Serverbound>> + 'static,
    >(
        clientbound_a_to_b: F1,
        serverbound_a_to_b: F2,
        clientbound_b_to_a: F3,
        serverbound_b_to_a: F4,
    ) -> Self {
        Self::new_using_boxed(
            Box::new(clientbound_a_to_b),
            Box::new(serverbound_a_to_b),
            Box::new(clientbound_b_to_a),
            Box::new(serverbound_b_to_a),
        )
    }

    /// Create a new [`PacketTranslator`] using the provided [`Box`]ed
    /// functions.
    #[inline]
    #[must_use]
    #[expect(clippy::type_complexity, reason = "Function Traits")]
    pub const fn new_using_boxed(
        clientbound_a_to_b: Box<
            dyn FnMut(VersionPacket<A, Clientbound>) -> Option<VersionPacket<B, Clientbound>>,
        >,
        serverbound_a_to_b: Box<
            dyn FnMut(VersionPacket<A, Serverbound>) -> Option<VersionPacket<B, Serverbound>>,
        >,
        clientbound_b_to_a: Box<
            dyn FnMut(VersionPacket<B, Clientbound>) -> Option<VersionPacket<A, Clientbound>>,
        >,
        serverbound_b_to_a: Box<
            dyn FnMut(VersionPacket<B, Serverbound>) -> Option<VersionPacket<A, Serverbound>>,
        >,
    ) -> Self {
        Self { clientbound_a_to_b, serverbound_a_to_b, clientbound_b_to_a, serverbound_b_to_a }
    }

    /// Convert a packet from [`Version`] `A` to `B`.
    ///
    /// Returns `None` if the conversion fails.
    #[must_use]
    pub fn try_a_to_b(
        &mut self,
        packet: VersionPacketBidirectional<A>,
    ) -> Option<VersionPacketBidirectional<B>> {
        match packet {
            VersionPacketBidirectional::Clientbound(packet) => {
                self.try_clientbound_a_to_b(packet).map(VersionPacketBidirectional::Clientbound)
            }
            VersionPacketBidirectional::Serverbound(packet) => {
                self.try_serverbound_a_to_b(packet).map(VersionPacketBidirectional::Serverbound)
            }
        }
    }

    /// Convert a packet from [`Version`] `B` to `A`.
    ///
    /// Returns `None` if the conversion fails.
    #[must_use]
    pub fn try_b_to_a(
        &mut self,
        packet: VersionPacketBidirectional<B>,
    ) -> Option<VersionPacketBidirectional<A>> {
        match packet {
            VersionPacketBidirectional::Clientbound(packet) => {
                self.try_clientbound_b_to_a(packet).map(VersionPacketBidirectional::Clientbound)
            }
            VersionPacketBidirectional::Serverbound(packet) => {
                self.try_serverbound_b_to_a(packet).map(VersionPacketBidirectional::Serverbound)
            }
        }
    }

    /// Convert a [`Clientbound`] packet from [`Version`] `A` to `B`.
    ///
    /// Returns `None` if the conversion fails.
    #[inline]
    #[must_use]
    pub fn try_clientbound_a_to_b(
        &mut self,
        packet: VersionPacket<A, Clientbound>,
    ) -> Option<VersionPacket<B, Clientbound>> {
        (self.clientbound_a_to_b)(packet)
    }

    /// Convert a [`Serverbound`] packet from [`Version`] `A` to `B`.
    ///
    /// Returns `None` if the conversion fails.
    #[inline]
    #[must_use]
    pub fn try_serverbound_a_to_b(
        &mut self,
        packet: VersionPacket<A, Serverbound>,
    ) -> Option<VersionPacket<B, Serverbound>> {
        (self.serverbound_a_to_b)(packet)
    }

    /// Convert a [`Clientbound`] packet from [`Version`] `B` to `A`.
    ///
    /// Returns `None` if the conversion fails.
    #[inline]
    #[must_use]
    pub fn try_clientbound_b_to_a(
        &mut self,
        packet: VersionPacket<B, Clientbound>,
    ) -> Option<VersionPacket<A, Clientbound>> {
        (self.clientbound_b_to_a)(packet)
    }

    /// Convert a [`Serverbound`] packet from [`Version`] `B` to `A`.
    ///
    /// Returns `None` if the conversion fails.
    #[inline]
    #[must_use]
    pub fn try_serverbound_b_to_a(
        &mut self,
        packet: VersionPacket<B, Serverbound>,
    ) -> Option<VersionPacket<A, Serverbound>> {
        (self.serverbound_b_to_a)(packet)
    }
}
