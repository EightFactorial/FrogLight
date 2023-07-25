use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundUpdateCommandBlockC2SPacket {
    pub a: BlockPos,
    pub b: String,
    pub c: Enum,
    pub d: u8,
}
