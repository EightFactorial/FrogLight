use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 1, 4, 116, 101, 115, 116, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct BookUpdateC2SPacket {
    #[frog(var)]
    pub slot: u32,
    pub pages: Vec<CompactString>,
    pub title: Option<CompactString>,
}
