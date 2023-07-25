use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundQueryBlockNbtC2SPacket {
    pub a: u32,
    pub b: BlockPos,
}
