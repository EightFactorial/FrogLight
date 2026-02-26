//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:custom_payload"

use froglight_common::prelude::Identifier;

use crate::common::unsized_buffer::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct CustomPayloadS2CPacket {
    pub identifier: Identifier<'static>,
    pub buffer: UnsizedBuffer<'static>,
}
