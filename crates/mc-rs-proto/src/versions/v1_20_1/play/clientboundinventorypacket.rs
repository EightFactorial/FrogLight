use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundInventoryPacket {
    pub a: u16,
    pub b: u32,
    pub c: Vec,
    pub d: ItemStack,
}
