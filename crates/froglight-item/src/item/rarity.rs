#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;

/// An item's rarity level.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq, Hash))]
pub enum ItemRarity {
    /// Common rarity.
    #[default]
    Common = 0,
    /// Uncommon rarity.
    Uncommon = 1,
    /// Rare rarity.
    Rare = 2,
    /// Epic rarity.
    Epic = 3,
}
