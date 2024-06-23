use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClickSlotPacket {
    pub container_id: u8,
    #[frog(var)]
    pub state_id: u32,
    pub slot_id: u16,
    pub button_id: u8,
    // TODO: Implement ClickSlotData
    pub data: UnsizedBuffer,
}
