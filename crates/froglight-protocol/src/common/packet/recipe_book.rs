use froglight_macros::FrogReadWrite;

/// The type of recipe book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum RecipeBookType {
    /// Crafting table recipes.
    CraftingTable,
    /// Furnace recipes.
    Furnace,
    /// Blast furnace recipes.
    BlastFurnace,
    /// Smoker recipes.
    Smoker,
}
