use froglight_macros::FrogReadWrite;
use glam::DVec3;

use crate::common::InteractionHand;

/// The kind of interaction a player is performing
#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [1])]
pub enum PlayerInteraction {
    /// The player is interacting with the entity
    Interact(InteractionHand),
    /// The player is attacking the entity
    Attack,
    /// The player is interacting with the entity at a specific position
    InteractAt(DVec3, InteractionHand),
}
