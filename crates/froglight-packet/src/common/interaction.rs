#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum PlayerInteraction {
    Interact(InteractionHand),
    Attack,
    InteractAt(InteractionAt),
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct InteractionAt {
    pub position: Vec3,
    pub hand: InteractionHand,
}

// -------------------------------------------------------------------------------------------------

/// The hand used for an interaction.
///
/// This is *not* the same as the player's right or left hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum InteractionHand {
    /// The player's main hand.
    Main,
    /// The player's offhand.
    Offhand,
}
