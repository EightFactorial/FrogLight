use bevy_math::DVec3;
use froglight_macros::FrogReadWrite;

/// The kind of interaction a player is performing
#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum InteractionAction {
    /// The player is interacting with the entity
    Interact(InteractionHand),
    /// The player is attacking the entity
    Attack,
    /// The player is interacting with the entity at a specific position
    InteractAt(DVec3, InteractionHand),
}

/// The hand the player is using to interact with
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum InteractionHand {
    /// The player's main hand
    #[default]
    MainHand,
    /// The player's off hand
    OffHand,
}
