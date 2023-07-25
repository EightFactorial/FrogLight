use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCreativeInventoryActionC2SPacket {
    pub a: u16,
    pub b: ItemStack,
}
