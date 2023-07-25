use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundScreenHandlerSlotUpdateS2CPacket {
    pub a: u8,
    pub b: u32,
    pub c: u16,
    pub d: ItemStack,
}
