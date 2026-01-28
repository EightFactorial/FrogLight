//! @generated [`Play`](crate::version::Play) packets for v26.1.x

// -------------------------------------------------------------------------------------------------

use froglight_common::version::V26_1;

use crate::version::*;

impl PacketState<V26_1> for Play {
    type Clientbound = ClientboundPackets;
    type Serverbound = ServerboundPackets;

    fn transition_state_to(_: &Self::Serverbound) -> Option<PacketStateEnum> { None }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    Placeholder = 0x00,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    Placeholder = 0x00,
}
