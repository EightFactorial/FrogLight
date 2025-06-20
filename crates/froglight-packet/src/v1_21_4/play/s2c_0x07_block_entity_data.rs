//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @manual @generated by {COMMIT_HASH}

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_nbt::nbt::UnnamedNbt;
use froglight_world::position::BlockPos;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct BlockEntityUpdateS2CPacket {
    pub position: BlockPos,
    #[cfg_attr(feature = "io", frog(var))]
    pub entity_type: u32,
    pub block_data: UnnamedNbt,
}
