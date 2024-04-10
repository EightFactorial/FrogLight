use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
pub struct WorldBorderWarningTimeChangedS2CPacket {
    #[frog(var)]
    pub time: u32,
}
