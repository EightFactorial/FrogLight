use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSetCameraEntityPacket {
    pub a: u32,
}
