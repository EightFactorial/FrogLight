//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:custom_query_answer"

#[cfg(feature = "facet")]
use facet_minecraft as mc;

use crate::common::unsized_buffer::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct CustomQueryAnswerC2SPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub query_id: u32,
    pub payload: Option<UnsizedBuffer<'static>>,
}
