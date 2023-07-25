use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerSpawnPacket {
    pub a: u32,
    pub b: Uuid,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: u8,
    pub g: u8,
}
