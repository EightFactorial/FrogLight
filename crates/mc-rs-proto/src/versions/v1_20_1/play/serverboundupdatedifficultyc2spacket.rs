use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundUpdateDifficultyC2SPacket {
    pub a: u16,
}
