use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInteractBlockC2SPacket {
    pub a: Enum,
    pub b: BlockHitResult,
    pub c: u32,
}
