use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 4, 84, 101, 115, 116])]
pub struct ScoreboardDisplayS2CPacket {
    pub slot: u8,
    pub name: CompactString,
}
