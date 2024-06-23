use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use serde_json::Value;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ScoreboardScoreUpdatePacket {
    pub entity_name: CompactString,
    pub objective_name: CompactString,
    #[frog(var)]
    pub value: u32,
    // TODO: Text
    pub display_name: Option<Value>,
    // TODO: Implement NumberFormat
    pub number_format: Option<UnsizedBuffer>,
}
