use mc_rs_macros::Transcode;

use crate::types::inventory::ItemSlot;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCreativeInventoryActionPacket {
    pub slot_id: u16,
    pub item: ItemSlot,
}
