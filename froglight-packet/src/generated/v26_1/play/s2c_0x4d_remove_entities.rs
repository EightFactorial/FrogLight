//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:remove_entities"

use alloc::vec::Vec;

use froglight_entity::prelude::EntityId;
#[cfg(feature = "facet")]
use froglight_facet as mc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RemoveEntitiesS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entities: Vec<EntityId>,
}
