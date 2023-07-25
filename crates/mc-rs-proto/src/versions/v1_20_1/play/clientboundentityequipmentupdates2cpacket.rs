use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityEquipmentUpdateS2CPacket {
    pub a: u32,
    pub b: u8,
    pub c: ItemStack,
}
