//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:set_entity_motion"

use froglight_common::prelude::EntityId;
#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::common::lpdvec3::LpDVec3;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SetEntityMotionS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_id: EntityId,
    pub delta: LpDVec3,
}
