use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundWorldBorderInitializeS2CPacket {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: u64,
    pub f: u32,
    pub g: u32,
    pub h: u32,
}
