use froglight_macros::FrogReadWrite;

/// An action that can be performed on items.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum ItemSlotAction {
    /// Pick up an item.
    #[default]
    Pickup,
    /// Quick move items.
    QuickMove,
    /// Swap items.
    Swap,
    /// Clone items.
    Clone,
    /// Throw items.
    Throw,
    /// Quick craft.
    QuickCraft,
    /// Pick up all items.
    PickupAll,
}
