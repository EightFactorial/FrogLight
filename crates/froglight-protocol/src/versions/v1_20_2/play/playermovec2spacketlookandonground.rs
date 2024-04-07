use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerMoveC2SPacketLookAndOnGround {
    pub field_0: (),
    pub field_1: (),
    pub field_2: (),
}
