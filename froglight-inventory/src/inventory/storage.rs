/// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InventoryStorage {}

impl InventoryStorage {
    /// Create a new, empty [`InventoryStorage`].
    #[must_use]
    pub const fn new() -> Self { Self {} }
}
