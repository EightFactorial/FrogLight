use mc_rs_macros::Transcode;
use hashbrown::HashMap;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundClickSlotC2SPacket {
    pub a: u8,
    pub b: u32,
    pub c: u16,
    pub d: u8,
    pub e: Enum,
    pub f: HashMap,
    pub g: ItemStack,
}
