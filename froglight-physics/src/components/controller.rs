//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

#[allow(unused_imports, reason = "May be used depending on features")]
use crate::prelude::*;

/// A controller for physics entities.
///
/// Allows for performing inputs and other actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(PhysicsState))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, Component))]
pub struct PhysicsController {
    inputs: u8,
}

impl Default for PhysicsController {
    fn default() -> Self { Self::new() }
}

impl PhysicsController {
    const INPUT_JUMP: u8 = 0b0000_0001;
    const INPUT_MOVE_BACKWARD: u8 = 0b0100_0000;
    const INPUT_MOVE_FORWARD: u8 = 0b1000_0000;
    const INPUT_MOVE_LEFT: u8 = 0b0010_0000;
    const INPUT_RIGHT: u8 = 0b0001_0000;
    const INPUT_SNEAK: u8 = 0b0000_0010;
    const INPUT_SPRINT: u8 = 0b0000_0100;

    /// Create a new [`PhysicsController`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self { inputs: 0 } }

    /// Returns `true` if the controller is moving forward.
    #[must_use]
    pub const fn is_moving_forward(&self) -> bool { self.inputs & Self::INPUT_MOVE_FORWARD != 0 }

    /// Set whether the controller is moving forward.
    pub const fn forward(&mut self, forward: bool) {
        if forward {
            self.inputs |= Self::INPUT_MOVE_FORWARD;
        } else {
            self.inputs &= !Self::INPUT_MOVE_FORWARD;
        }
    }

    /// Returns `true` if the controller is moving backward.
    #[must_use]
    pub const fn is_moving_backward(&self) -> bool { self.inputs & Self::INPUT_MOVE_BACKWARD != 0 }

    /// Set whether the controller is moving backward.
    pub const fn backward(&mut self, backward: bool) {
        if backward {
            self.inputs |= Self::INPUT_MOVE_BACKWARD;
        } else {
            self.inputs &= !Self::INPUT_MOVE_BACKWARD;
        }
    }

    /// Returns `true` if the controller is moving left.
    #[must_use]
    pub const fn is_moving_left(&self) -> bool { self.inputs & Self::INPUT_MOVE_LEFT != 0 }

    /// Set whether the controller is moving left.
    pub const fn left(&mut self, left: bool) {
        if left {
            self.inputs |= Self::INPUT_MOVE_LEFT;
        } else {
            self.inputs &= !Self::INPUT_MOVE_LEFT;
        }
    }

    /// Returns `true` if the controller is moving right.
    #[must_use]
    pub const fn is_moving_right(&self) -> bool { self.inputs & Self::INPUT_RIGHT != 0 }

    /// Set whether the controller is moving right.
    pub const fn right(&mut self, right: bool) {
        if right {
            self.inputs |= Self::INPUT_RIGHT;
        } else {
            self.inputs &= !Self::INPUT_RIGHT;
        }
    }

    /// Returns `true` if the controller is jumping.
    #[must_use]
    pub const fn is_jumping(&self) -> bool { self.inputs & Self::INPUT_JUMP != 0 }

    /// Set whether the controller is jumping.
    pub const fn jump(&mut self, jump: bool) {
        if jump {
            self.inputs |= Self::INPUT_JUMP;
        } else {
            self.inputs &= !Self::INPUT_JUMP;
        }
    }

    /// Returns `true` if the controller is sneaking.
    #[must_use]
    pub const fn is_sneaking(&self) -> bool { self.inputs & Self::INPUT_SNEAK != 0 }

    /// Set whether the controller is sneaking.
    pub const fn sneak(&mut self, sneak: bool) {
        if sneak {
            self.inputs |= Self::INPUT_SNEAK;
        } else {
            self.inputs &= !Self::INPUT_SNEAK;
        }
    }

    /// Returns `true` if the controller is sprinting.
    #[must_use]
    pub const fn is_sprinting(&self) -> bool { self.inputs & Self::INPUT_SPRINT != 0 }

    /// Set whether the controller is sprinting.
    pub const fn sprint(&mut self, sprint: bool) {
        if sprint {
            self.inputs |= Self::INPUT_SPRINT;
        } else {
            self.inputs &= !Self::INPUT_SPRINT;
        }
    }
}
