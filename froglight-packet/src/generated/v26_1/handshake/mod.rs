//! @generated [`Handshake`](crate::version::Handshake) packets for v26.1.x

pub mod c2s_0x00_intention;
pub use c2s_0x00_intention::HandshakeC2SPacket;
// -------------------------------------------------------------------------------------------------
use froglight_common::version::V26_1;

use crate::{
    core::{impossible::Impossible, intent::ConnectionIntent},
    version::*,
};

impl PacketState<V26_1> for Handshake {
    type Clientbound = ClientboundPackets;
    type Serverbound = ServerboundPackets;

    fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {
        let ServerboundPackets::Handshake(HandshakeC2SPacket { intent, .. }) = packet;
        match intent {
            ConnectionIntent::Status => Some(PacketStateEnum::Status),
            ConnectionIntent::Login => Some(PacketStateEnum::Login),
            ConnectionIntent::Transfer => None,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    None(Impossible) = 0x00,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    Handshake(HandshakeC2SPacket) = 0x00,
}
