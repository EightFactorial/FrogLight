use froglight_macros::FrogReadWrite;

use crate::packet::RecipeBookCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
pub struct RecipeCategoryOptionsC2SPacket {
    pub category: RecipeBookCategory,
    pub gui_open: bool,
    pub filtering_craftable: bool,
}
