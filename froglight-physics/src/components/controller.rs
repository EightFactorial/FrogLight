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
#[cfg_attr(feature = "bevy", derive(Component, Reflect), require(PhysicsState))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, Component))]
pub struct PhysicsController {
    input: ControllerInputFlags,
}

impl Default for PhysicsController {
    fn default() -> Self { Self::new() }
}

impl PhysicsController {
    /// Create a new [`PhysicsController`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self { input: ControllerInputFlags::empty() } }

    /// Get the current input flags for this controller.
    #[inline]
    #[must_use]
    pub const fn get_inputs(&self) -> ControllerInputFlags { self.input }

    /// Set the input flags for this controller.
    #[inline]
    pub const fn set_inputs(&mut self, input: ControllerInputFlags) { self.input = input; }

    /// Set the forward input for this controller.
    #[inline]
    pub fn set_move_forward(&mut self, enabled: bool) {
        self.input.set(ControllerInputFlags::MOVE_FORWARD, enabled);
    }

    /// Set the backward input for this controller.
    #[inline]
    pub fn set_move_backward(&mut self, enabled: bool) {
        self.input.set(ControllerInputFlags::MOVE_BACKWARD, enabled);
    }

    /// Set the left input for this controller.
    #[inline]
    pub fn set_move_left(&mut self, enabled: bool) {
        self.input.set(ControllerInputFlags::MOVE_LEFT, enabled);
    }

    /// Set the right input for this controller.
    #[inline]
    pub fn set_move_right(&mut self, enabled: bool) {
        self.input.set(ControllerInputFlags::MOVE_RIGHT, enabled);
    }

    /// Set the jump input for this controller.
    #[inline]
    pub fn set_jump(&mut self, enabled: bool) {
        self.input.set(ControllerInputFlags::JUMP, enabled);
    }

    /// Set the sneak input for this controller.
    #[inline]
    pub fn set_sneak(&mut self, enabled: bool) {
        self.input.set(ControllerInputFlags::SNEAK, enabled);
    }

    /// Set the sprint input for this controller.
    #[inline]
    pub fn set_sprint(&mut self, enabled: bool) {
        self.input.set(ControllerInputFlags::SPRINT, enabled);
    }
}

// -------------------------------------------------------------------------------------------------

bitflags::bitflags! {
    /// The input flags for a [`PhysicsController`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque))]
    #[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
    pub struct ControllerInputFlags: u8 {
        const MOVE_FORWARD = 0b0000_0001;
        const MOVE_BACKWARD = 0b0000_0010;
        const MOVE_LEFT = 0b0000_0100;
        const MOVE_RIGHT = 0b0000_1000;
        const JUMP = 0b0001_0000;
        const SNEAK = 0b0010_0000;
        const SPRINT = 0b0100_0000;
    }
}
