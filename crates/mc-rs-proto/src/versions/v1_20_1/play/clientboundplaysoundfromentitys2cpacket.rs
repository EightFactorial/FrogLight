use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlaySoundFromEntityS2CPacket {
    pub a: RegistryEntry,
    pub b: Enum,
    pub c: u32,
    pub d: f32,
    pub e: f32,
    pub f: u64,
}
