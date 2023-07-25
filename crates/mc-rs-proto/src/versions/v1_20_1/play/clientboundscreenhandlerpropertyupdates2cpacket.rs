use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScreenHandlerPropertyUpdateS2CPacket {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}
