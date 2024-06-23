use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
pub struct CreativeInventoryActionPacket {
    pub slot_id: u16,
    // TODO: Implement ItemData
    pub data: UnsizedBuffer,
}
