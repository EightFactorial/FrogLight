use mc_rs_macros::Transcode;

use crate::types::inventory::ItemSlot;

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 2, 0])]
pub struct ClientboundScreenHandlerSlotUpdatePacket {
    pub container_id: i8,
    #[var]
    pub state_id: u32,
    pub slot_id: u16,
    pub item: ItemSlot,
}
