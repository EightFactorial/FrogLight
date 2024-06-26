use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ScoreboardScoreUpdatePacket {
    pub entity_name: CompactString,
    pub objective_name: CompactString,
    #[frog(var)]
    pub value: u32,
    pub display_name: Option<NbtTag>,
    // TODO: Implement NumberFormat
    pub number_format: Option<UnsizedBuffer>,
}
