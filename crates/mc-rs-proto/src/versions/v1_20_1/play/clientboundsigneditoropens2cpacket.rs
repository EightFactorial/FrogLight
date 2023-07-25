use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSignEditorOpenS2CPacket {
    pub a: BlockPos,
    pub b: bool,
}
