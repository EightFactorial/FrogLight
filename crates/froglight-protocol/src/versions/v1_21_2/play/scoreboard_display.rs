//! @generated by `froglight-generator` #8ddd9f0

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ScoreboardDisplayPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: CompactString,
}
