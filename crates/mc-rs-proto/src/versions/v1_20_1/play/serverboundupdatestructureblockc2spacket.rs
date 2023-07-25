use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateStructureBlockC2SPacket {
    pub a: BlockPos,
    pub b: Enum,
    pub c: Enum,
    pub d: String,
    pub e: u8,
    pub f: u8,
    pub g: u8,
    pub h: u8,
    pub i: u8,
    pub j: u8,
    pub k: Enum,
    pub l: Enum,
    pub m: String,
    pub n: f32,
    pub o: u64,
    pub p: u8,
}
