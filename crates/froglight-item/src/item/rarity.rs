#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// An item's rarity level.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq, Hash))]
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
