#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// A player statistic type.
///
/// Does not include the value of the statistic, only the type and ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct PlayerStatisticType {
    /// The type of statistic.
    pub kind: PlayerStatisticKind,
    /// The statistic ID.
    ///
    /// Varies by kind of statistic.
    #[cfg_attr(feature = "io", frog(var))]
    pub statistic_id: u32,
}

// -------------------------------------------------------------------------------------------------

/// The type of player statistic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum PlayerStatisticKind {
    Mined,
    Crafted,
    Used,
    Broken,
    PickedUp,
    Dropped,
    Killed,
    KilledBy,
    Custom,
}
