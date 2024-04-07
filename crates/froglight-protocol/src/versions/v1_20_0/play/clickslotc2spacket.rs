use froglight_macros::FrogReadWrite;
use hashbrown::HashMap;

use crate::common::{ItemAction, LegacyItemSlot};

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ClickSlotC2SPacket {
    pub container_id: u8,
    #[frog(var)]
    pub state_id: u32,
    pub slot_id: i16,
    pub button_id: u8,
    pub item_action: ItemAction,
    pub changed_slots: HashMap<u16, LegacyItemSlot>,
    pub carried_item: LegacyItemSlot,
}
