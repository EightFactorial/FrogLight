use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExperienceOrbSpawnS2CPacket {
    pub a: u32,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: u16,
}
