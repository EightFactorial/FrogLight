use std::fmt::Debug;

use bevy_reflect::Reflect;
use froglight_core::common::ResourceKey;
use froglight_protocol::traits::Version;

/// A block type
pub trait BlockType<V: Version>: Debug + Reflect {
    /// Get the [`ResourceKey`] of the block.
    fn resource_key(&self) -> ResourceKey;

    /// Get the block from the relative state id.
    ///
    /// ---
    ///
    /// The relative state id is the state id minus the minimum state id.
    ///
    /// For example, if the minimum state id is `2` and the state id is `4`,
    /// the relative state id is `4 - 2 = 2`.
    #[allow(clippy::wrong_self_convention)]
    fn from_relative_state(&self, id: usize) -> Option<Self>
    where
        Self: std::marker::Sized;

    /// Get the total number of states for the block.
    ///
    /// ---
    ///
    /// This is equivalent to the number of states each block attribute has
    /// multiplied together.
    ///
    /// For example, if a block has 3 attributes with 4, 2, and 3 states
    /// respectively, the total number of states is `4 * 2 * 3 = 24`.
    fn states(&self) -> usize { 1 }

    /// Returns `true` if the block is air.
    fn is_air(&self) -> bool { false }
    /// Returns `true` if the block is opaque.
    fn is_opaque(&self) -> bool { true }
    /// Returns `true` if the block is collidable.
    fn is_collidable(&self) -> bool { true }
}
