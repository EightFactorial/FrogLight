use mc_rs_macros::Packet;
use hashbrown::HashMap;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundClickSlotC2SPacket {
    pub a: u8,
    pub b: u32,
    pub c: u16,
    pub d: u8,
    pub e: Enum,
    pub f: HashMap,
    pub g: ItemStack,
}
