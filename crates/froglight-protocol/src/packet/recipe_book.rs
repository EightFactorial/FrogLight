use froglight_macros::FrogReadWrite;

/// The type of recipe book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
pub enum RecipeBookCategory {
    /// Crafting table recipes.
    CraftingTable,
    /// Furnace recipes.
    Furnace,
    /// Blast furnace recipes.
    BlastFurnace,
    /// Smoker recipes.
    Smoker,
}
