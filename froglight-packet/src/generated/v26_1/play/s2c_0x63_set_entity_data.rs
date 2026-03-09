//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:set_entity_data"

use crate::common::{entity_id::VarEntityId, unsized_buffer::UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SetEntityDataS2CPacket {
    pub entity_id: VarEntityId,
    pub metadata: UnsizedBuffer<'static>,
}
