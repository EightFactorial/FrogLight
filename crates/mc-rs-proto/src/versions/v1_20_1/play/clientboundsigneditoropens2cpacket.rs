use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSignEditorOpenS2CPacket {
    pub a: BlockPos,
    pub b: bool,
}
