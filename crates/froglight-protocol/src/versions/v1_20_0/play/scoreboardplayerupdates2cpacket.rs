use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ScoreboardPlayerUpdateS2CPacket {
    pub player_name: (),
    pub mode: (),
    pub objective_name: (),
    pub score: (),
}
