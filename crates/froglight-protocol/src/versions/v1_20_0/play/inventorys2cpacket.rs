use froglight_macros::FrogReadWrite;

use crate::packet::LegacyItemSlot;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 2, 0, 0, 0])]
pub struct InventoryS2CPacket {
    pub container_id: u8,
    #[frog(var)]
    pub state_id: u32,
    pub items: Vec<LegacyItemSlot>,
    pub held_item: LegacyItemSlot,
}
