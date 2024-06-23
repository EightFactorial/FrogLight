use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ScoreboardScoreResetPacket {
    pub entity_name: CompactString,
    pub objective: Option<CompactString>,
}
