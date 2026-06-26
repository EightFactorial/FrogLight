//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:set_entity_data"

use froglight_entity::prelude::EntityId;
#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::common::unsized_buffer::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SetEntityDataS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_id: EntityId,
    pub metadata: UnsizedBuffer<'static>,
}
