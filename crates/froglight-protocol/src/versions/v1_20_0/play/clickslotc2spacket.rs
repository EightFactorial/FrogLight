use froglight_macros::FrogReadWrite;
use hashbrown::HashMap;

use crate::packet::{ItemSlotAction, LegacyItemSlot};

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClickSlotC2SPacket {
    pub container_id: u8,
    #[frog(var)]
    pub state_id: u32,
    pub slot_id: i16,
    pub button_id: u8,
    pub item_action: ItemSlotAction,
    pub changed_slots: HashMap<u16, LegacyItemSlot>,
    pub carried_item: LegacyItemSlot,
}
