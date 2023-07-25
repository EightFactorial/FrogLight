use mc_rs_macros::Packet;
use uuid::Uuid;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntitySpawnS2CPacket {
    pub a: u32,
    pub b: Uuid,
    pub c: Object,
    pub d: f64,
    pub e: f64,
    pub f: f64,
    pub g: u8,
    pub h: u8,
    pub i: u8,
    pub j: u32,
    pub k: u16,
    pub l: u16,
    pub m: u16,
}
