use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RecipeCategoryOptionsC2SPacket {
    pub category: (),
    pub gui_open: (),
    pub filtering_craftable: (),
}
