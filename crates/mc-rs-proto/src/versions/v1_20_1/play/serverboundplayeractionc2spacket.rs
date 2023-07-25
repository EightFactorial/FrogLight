use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerActionC2SPacket {
    pub a: Enum,
    pub b: BlockPos,
    pub c: u16,
    pub d: u32,
}
