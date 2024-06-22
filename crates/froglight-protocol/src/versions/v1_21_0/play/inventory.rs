use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct InventoryPacket {
    pub container_id: u8,
    #[frog(var)]
    pub state_id: u32,
    // TODO: Implement Inventory
    pub data: UnsizedBuffer,
}
