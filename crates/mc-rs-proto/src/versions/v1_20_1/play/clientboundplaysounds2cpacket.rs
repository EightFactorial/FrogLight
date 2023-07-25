use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlaySoundS2CPacket {
    pub a: RegistryEntry,
    pub b: Enum,
    pub c: u32,
    pub d: u32,
    pub e: u32,
    pub f: f32,
    pub g: f32,
    pub h: u64,
}
