use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerActionC2SPacket {
    pub action: (),
    pub pos: (),
    pub direction: (),
    pub sequence: (),
}
