use froglight_macros::FrogReadWrite;

use crate::packet::LegacyItemSlot;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 2, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ScreenHandlerSlotUpdateS2CPacket {
    pub container_id: u8,
    #[frog(var)]
    pub revision: u32,
    pub slot: u16,
    pub stack: LegacyItemSlot,
}
