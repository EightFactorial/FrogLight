use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldEventPacket {
    pub a: u32,
    pub b: BlockPos,
    pub c: u32,
    pub d: bool,
}
