use froglight_macros::FrogReadWrite;

use crate::common::LegacyItemSlot;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CreativeInventoryActionC2SPacket {
    pub slot: u16,
    pub stack: LegacyItemSlot,
}
