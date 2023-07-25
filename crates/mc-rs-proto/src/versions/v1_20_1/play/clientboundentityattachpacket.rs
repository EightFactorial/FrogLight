use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAttachPacket {
    pub a: u32,
    pub b: u32,
}
