use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [2, 77, 67, 1])]
pub struct GameMessageS2CPacket {
    pub message: CompactString,
    pub overlay: bool,
}
