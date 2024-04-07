use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CraftRequestC2SPacket {
    pub sync_id: (),
    pub recipe: (),
    pub craft_all: (),
}
