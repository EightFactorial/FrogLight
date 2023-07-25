use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundGameJoinPacket {
    pub a: u32,
    pub b: bool,
    pub c: u8,
    pub d: u8,
    pub e: Vec,
    pub f: Object,
    pub g: RegistryKey,
    pub h: RegistryKey,
    pub i: u64,
    pub j: u32,
    pub k: u32,
    pub l: u32,
    pub m: bool,
    pub n: bool,
    pub o: bool,
    pub p: bool,
    pub q: Option,
    pub r: u32,
}
