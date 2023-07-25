use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityPositionS2CPacket {
    pub a: u32,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: u8,
    pub f: u8,
    pub g: bool,
}
