use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlaySoundFromEntityPacket {
    pub a: RegistryEntry,
    pub b: Enum,
    pub c: u32,
    pub d: f32,
    pub e: f32,
    pub f: u64,
}
