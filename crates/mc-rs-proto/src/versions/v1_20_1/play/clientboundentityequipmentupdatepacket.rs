use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityEquipmentUpdatePacket {
    pub a: u32,
    pub b: u8,
    pub c: ItemStack,
}
