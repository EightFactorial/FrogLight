use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundCreativeInventoryActionC2SPacket {
    pub a: u16,
    pub b: ItemStack,
}
