use froglight_macros::FrogReadWrite;

/// An action the client can perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [4])]
pub enum ClientPlayerAction {
    /// The player is destroying a block.
    StartDestroyBlock,
    /// The player stopped destroying a block.
    AbortDestroyBlock,
    /// The player has destroyed a block.
    StopDestroyBlock,
    /// The player has dropped all items in a stack.
    DropAllItems,
    /// The player has dropped an item.
    DropItem,
    /// The player stopped using an item.
    ReleaseUseItem,
    /// The player swapped an item with their offhand.
    SwapItemWithOffhand,
}
