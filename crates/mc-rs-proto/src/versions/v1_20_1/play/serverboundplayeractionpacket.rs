use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerActionPacket {
    pub a: Enum,
    pub b: BlockPos,
    pub c: u16,
    pub d: u32,
}
