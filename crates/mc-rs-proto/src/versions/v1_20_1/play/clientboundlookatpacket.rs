use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLookAtPacket {
    pub a: Enum,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: bool,
    pub f: u32,
    pub g: Enum,
}
