use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScreenHandlerSlotUpdatePacket {
    pub a: u8,
    pub b: u32,
    pub c: u16,
    pub d: ItemStack,
}
