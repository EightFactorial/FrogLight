use mc_rs_macros::Transcode;

use crate::types::inventory::ItemSlot;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundInventoryPacket {
    pub container_id: i8,
    #[var]
    pub state_id: u32,
    pub items: Vec<ItemSlot>,
    pub held_item: ItemSlot,
}
