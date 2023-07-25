use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntitySetHeadYawPacket {
    pub a: u32,
    pub b: u8,
}
