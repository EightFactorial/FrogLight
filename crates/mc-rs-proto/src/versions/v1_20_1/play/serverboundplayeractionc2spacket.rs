use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerActionC2SPacket {
    pub a: Enum,
    pub b: BlockPos,
    pub c: u16,
    pub d: u32,
}
