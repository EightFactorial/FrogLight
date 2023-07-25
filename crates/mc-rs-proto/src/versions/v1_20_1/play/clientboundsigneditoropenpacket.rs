use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSignEditorOpenPacket {
    pub a: BlockPos,
    pub b: bool,
}
