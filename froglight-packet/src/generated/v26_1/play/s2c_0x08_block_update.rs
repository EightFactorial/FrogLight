//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:block_update"

use froglight_block::state::GlobalStateId;
#[cfg(feature = "facet")]
use froglight_facet as mc;
use froglight_world::component::BlockPos;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BlockUpdateS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::with = BlockPos::WITH_PACKED))]
    pub position: BlockPos,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub block_id: GlobalStateId,
}
