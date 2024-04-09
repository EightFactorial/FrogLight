use froglight_macros::FrogReadWrite;

use crate::packet::LegacyItemSlot;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
pub struct CreativeInventoryActionC2SPacket {
    pub slot: u16,
    pub stack: LegacyItemSlot,
}
