use mc_rs_macros::Packet;
use uuid::Uuid;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerSpawnS2CPacket {
    pub a: u32,
    pub b: Uuid,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: u8,
    pub g: u8,
}
