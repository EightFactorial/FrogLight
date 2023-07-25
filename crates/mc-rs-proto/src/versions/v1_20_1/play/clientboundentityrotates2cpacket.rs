use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityRotateS2CPacket {
    pub a: u32,
    pub b: u8,
    pub c: u8,
    pub d: bool,
}
