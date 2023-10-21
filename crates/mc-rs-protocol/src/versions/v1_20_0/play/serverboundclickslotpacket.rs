use hashbrown::HashMap;
use mc_rs_macros::Transcode;

use crate::types::{inventory::ItemSlot, packets::inventory::ClickType};

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundClickSlotPacket {
    pub container_id: u8,
    #[var]
    pub state_id: u32,
    pub slot_id: i16,
    pub button_id: u8,
    pub click_type: ClickType,
    pub changed_slots: HashMap<u16, ItemSlot>,
    pub g: ItemSlot,
}
