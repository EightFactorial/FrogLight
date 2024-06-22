//! @generated by `froglight-generator` #73eaa37

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UpdateSignPacket {
    pub field_0: BlockPosition,
    pub field_1: bool,
    pub field_2: CompactString,
}