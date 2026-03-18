//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:entity_position_sync"

use crate::common::{entity_id::VarEntityId, position::EntityPositionRotationData};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityPositionSyncS2CPacket {
    pub entity_id: VarEntityId,
    pub data: EntityPositionRotationData,
    pub on_ground: bool,
}
