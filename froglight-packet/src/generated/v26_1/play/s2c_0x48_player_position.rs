//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:player_position"

#[cfg(feature = "facet")]
use facet_minecraft as mc;

use crate::common::position::{EntityPositionRotationData, EntityRelativeFlags};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PlayerPositionS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub teleport_id: u32,
    pub data: EntityPositionRotationData,
    pub relative: EntityRelativeFlags,
}
