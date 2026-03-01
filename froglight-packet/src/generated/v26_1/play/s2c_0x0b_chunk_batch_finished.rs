//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:chunk_batch_finished"

#[cfg(feature = "facet")]
use facet_minecraft as mc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ChunkBatchFinishedS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub batch_size: u32,
}
