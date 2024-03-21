use compact_str::CompactString;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [4, 74, 65, 73, 74])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RenameItemC2SPacket {
    pub name: CompactString,
}
