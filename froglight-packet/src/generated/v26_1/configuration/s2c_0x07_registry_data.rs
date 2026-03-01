//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:registry_data"

use froglight_common::prelude::Identifier;

use crate::common::unsized_buffer::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RegistryDataS2CPacket {
    pub registry: Identifier<'static>,
    pub payload: UnsizedBuffer<'static>,
}
