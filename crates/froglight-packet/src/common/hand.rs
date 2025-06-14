#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use serde::{Deserialize, Serialize};

/// The player's primary hand.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    feature = "bevy",
    derive(Reflect),
    reflect(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)
)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum PlayerHand {
    /// Left-handed
    Left,
    /// Right-handed
    #[default]
    Right,
}
