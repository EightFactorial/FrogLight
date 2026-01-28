//! @generated [`Handshake`](crate::version::Handshake) packets for v26.1.x

// -------------------------------------------------------------------------------------------------

use froglight_common::version::V26_1;

use crate::{core::impossible::Impossible, version::*};

impl PacketState<V26_1> for Handshake {
    type Clientbound = ClientboundPackets;
    type Serverbound = ServerboundPackets;

    // TODO: Placeholder until `ServerboundPackets::Handshake` is implemented
    fn transition_state_to(packet: &Self::Serverbound) -> Option<PacketStateEnum> {
        let ServerboundPackets::Handshake(val) = packet;
        match val {
            1 => Some(PacketStateEnum::Status),
            2 => Some(PacketStateEnum::Login),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    Impossible(Impossible) = 0x00,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    Handshake(u32) = 0x00,
}
