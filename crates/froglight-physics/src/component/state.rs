//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// The ground and jumping state of an entity.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[expect(clippy::struct_excessive_bools)]
pub struct EntityGroundState {
    /// Whether the entity is jumping.
    is_jumping: bool,
    /// The number of ticks until the entity can jump again.
    jump_delay: u32,
    /// Whether the entity is falling.
    is_falling: bool,
    /// How far the entity has fallen.
    fall_distance: f32,
    /// Whether the entity is on the ground.
    is_standing: bool,
    /// Whether the entity was on the ground last tick.
    was_standing: bool,
}

impl EntityGroundState {
    /// Returns `true` if the entity is jumping.
    #[inline]
    #[must_use]
    pub const fn is_jumping(&self) -> bool { self.is_jumping }

    /// Returns `true` if the entity is falling.
    #[inline]
    #[must_use]
    pub const fn is_falling(&self) -> bool { self.is_falling }

    /// Returns `true` if the entity is standing on the ground.
    #[inline]
    #[must_use]
    pub const fn is_standing(&self) -> bool { self.is_standing }
}

impl Default for EntityGroundState {
    fn default() -> Self {
        Self {
            is_jumping: false,
            jump_delay: 0,
            is_falling: false,
            fall_distance: 0.0,
            is_standing: true,
            was_standing: true,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The state of a player in the physics simulation.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), require(EntityGroundState))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[expect(clippy::struct_excessive_bools)]
pub struct PlayerPhysicsState {
    /// Whether the player is crouching.
    is_crouching: bool,
    /// Whether the player is sprinting.
    is_sprinting: bool,
    /// Whether the player is swimming.
    is_swimming: bool,
    /// Whether the player is gliding.
    is_gliding: bool,
    /// Whether the player is flying.
    is_flying: bool,
}

impl PlayerPhysicsState {
    /// Returns `true` if the player is crouching.
    #[inline]
    #[must_use]
    pub const fn is_crouching(&self) -> bool { self.is_crouching }

    /// Returns `true` if the player is sprinting.
    #[inline]
    #[must_use]
    pub const fn is_sprinting(&self) -> bool { self.is_sprinting }

    /// Returns `true` if the player is swimming.
    #[inline]
    #[must_use]
    pub const fn is_swimming(&self) -> bool { self.is_swimming }

    /// Returns `true` if the player is gliding.
    #[inline]
    #[must_use]
    pub const fn is_gliding(&self) -> bool { self.is_gliding }

    /// Returns `true` if the player is flying.
    #[inline]
    #[must_use]
    pub const fn is_flying(&self) -> bool { self.is_flying }
}
