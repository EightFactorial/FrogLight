//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:move_entity_pos"

use crate::common::{entity_id::VarEntityId, position::PositionDelta};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct MoveEntityPosS2CPacket {
    pub entity_id: VarEntityId,
    pub delta: PositionDelta,
    pub on_ground: bool,
}
