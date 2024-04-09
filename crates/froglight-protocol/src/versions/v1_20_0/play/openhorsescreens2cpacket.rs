use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct OpenHorseScreenS2CPacket {
    pub container_id: u8,
    #[frog(var)]
    pub slot_count: u32,
    pub horse_id: EntityId,
}
