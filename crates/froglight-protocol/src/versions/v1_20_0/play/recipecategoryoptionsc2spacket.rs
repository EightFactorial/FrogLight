use froglight_macros::FrogReadWrite;

use crate::common::RecipeBookType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RecipeCategoryOptionsC2SPacket {
    pub category: RecipeBookType,
    pub gui_open: bool,
    pub filtering_craftable: bool,
}
