use mc_rs_macros::Transcode;

use crate::types::inventory::ItemSlot;

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0])]
pub struct ServerboundCreativeInventoryActionPacket {
    pub slot_id: u16,
    pub item: ItemSlot,
}
