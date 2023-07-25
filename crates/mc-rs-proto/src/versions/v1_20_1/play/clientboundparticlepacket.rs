use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundParticlePacket {
    pub a: Object,
    pub b: bool,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub i: f32,
    pub j: u32,
}
