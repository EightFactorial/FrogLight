use froglight_macros::FrogReadWrite;

use crate::packet::RecipeBookCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
pub struct RecipeCategoryOptionsPacket {
    pub recipe_book: RecipeBookCategory,
    pub book_open: bool,
    pub filter_active: bool,
}
