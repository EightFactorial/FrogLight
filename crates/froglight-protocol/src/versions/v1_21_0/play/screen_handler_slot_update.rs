use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ScreenHandlerSlotUpdatePacket {
    pub container_id: u8,
    #[frog(var)]
    pub revision: u32,
    pub slot: u16,
    // TODO: Implement ScreenHandlerSlotUpdate
    pub data: UnsizedBuffer,
}
