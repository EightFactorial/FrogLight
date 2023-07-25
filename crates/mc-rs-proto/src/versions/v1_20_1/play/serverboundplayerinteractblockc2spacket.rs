use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerInteractBlockC2SPacket {
    pub a: Enum,
    pub b: BlockHitResult,
    pub c: u32,
}
