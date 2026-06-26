//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:move_entity_pos"

use froglight_entity::prelude::EntityId;
#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::common::position::PositionDelta;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct MoveEntityPosS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_id: EntityId,
    pub delta: PositionDelta,
    pub on_ground: bool,
}
