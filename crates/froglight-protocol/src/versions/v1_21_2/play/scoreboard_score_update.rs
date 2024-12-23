//! @generated by `froglight-generator` #8ddd9f0

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ScoreboardScoreUpdatePacket {
    pub field_0: CompactString,
    pub field_1: CompactString,
    #[frog(var)]
    pub field_2: u32,
    pub field_3: Option<Text>,
    pub field_4: Option<()>,
}
