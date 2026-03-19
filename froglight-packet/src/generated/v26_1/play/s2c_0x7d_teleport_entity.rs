//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:teleport_entity"

use crate::common::{
    entity_id::VarEntityId,
    position::{EntityPositionRotationData, EntityRelativeFlags},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TeleportEntityS2CPacket {
    pub entity_id: VarEntityId,
    pub data: EntityPositionRotationData,
    pub relative: EntityRelativeFlags,
    pub on_ground: bool,
}
