//! @generated by `froglight-generator` #3ae6f0f

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ServerTransferPacket {
    pub field_0: CompactString,
    #[frog(var)]
    pub field_1: u32,
}
