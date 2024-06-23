use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 1, 4, 116, 101, 115, 116, 0])]
pub struct BookUpdatePacket {
    #[frog(var)]
    pub hotbar_slot: u32,
    pub pages: Vec<CompactString>,
    pub title: Option<CompactString>,
}
