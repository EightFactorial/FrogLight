use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ScoreboardObjectiveUpdateS2CPacket {
    pub name: (),
    pub mode: (),
    pub display_name: (),
    pub kind: (),
}
