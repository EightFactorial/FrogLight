//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:teleport_entity"

use froglight_common::prelude::EntityId;
#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::common::position::{EntityPositionRotationData, EntityRelativeFlags};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TeleportEntityS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_id: EntityId,
    pub data: EntityPositionRotationData,
    pub relative: EntityRelativeFlags,
    pub on_ground: bool,
}
