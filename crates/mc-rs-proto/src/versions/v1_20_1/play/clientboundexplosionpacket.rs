use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExplosionPacket {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f32,
    pub e: Vec,
    pub f: f32,
    pub g: f32,
    pub h: f32,
}
