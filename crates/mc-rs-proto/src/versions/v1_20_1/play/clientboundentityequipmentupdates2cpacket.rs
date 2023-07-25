use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityEquipmentUpdateS2CPacket {
    pub a: u32,
    pub b: u8,
    pub c: ItemStack,
}
