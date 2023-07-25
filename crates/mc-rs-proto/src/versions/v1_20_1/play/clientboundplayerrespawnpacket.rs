use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerRespawnPacket {
    pub a: RegistryKey,
    pub b: RegistryKey,
    pub c: u64,
    pub d: u16,
    pub e: u8,
    pub f: bool,
    pub g: bool,
    pub h: u8,
    pub i: Option,
    pub j: u32,
}
