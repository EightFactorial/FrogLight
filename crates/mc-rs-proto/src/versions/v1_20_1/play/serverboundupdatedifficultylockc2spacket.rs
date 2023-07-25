use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundUpdateDifficultyLockC2SPacket {
    pub a: bool,
}
