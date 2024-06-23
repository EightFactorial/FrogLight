use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 4, 84, 101, 115, 116])]
pub struct ScoreboardDisplayPacket {
    #[frog(var)]
    pub position: u32,
    pub objective: CompactString,
}
