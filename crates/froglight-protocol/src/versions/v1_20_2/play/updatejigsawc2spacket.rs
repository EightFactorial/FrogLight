use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct UpdateJigsawC2SPacket {
    pub pos: (),
    pub name: (),
    pub target: (),
    pub pool: (),
    pub final_state: (),
    pub joint_type: (),
}
