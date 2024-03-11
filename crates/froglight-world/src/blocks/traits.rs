use std::fmt::Debug;

use bevy_reflect::Reflect;
use froglight_core::common::ResourceKey;
use froglight_protocol::traits::Version;

use super::registry::InnerRegistry;

/// A block
pub trait BlockType<V: Version>: Debug + Reflect {
    /// Get the [`ResourceKey`] of the block.
    fn resource_key(&self) -> ResourceKey;

    /// Get the total number of states for the block.
    ///
    /// ---
    ///
    /// This is equivalent to the number of states each block attribute has
    /// multiplied together.
    ///
    /// For example, if a block has 3 attributes with 4, 2, and 3 states
    /// respectively, the total number of states is `4 * 2 * 3 = 24`.
    fn states(&self) -> u32 { 1 }

    /// Returns `true` if the block is air.
    fn is_air(&self) -> bool { false }
    /// Returns `true` if the block is opaque.
    fn is_opaque(&self) -> bool { true }
    /// Returns `true` if the block is collidable.
    fn is_collidable(&self) -> bool { true }
}

/// Extra methods for blocks.
pub trait BlockExt<V: Version>: Sized + BlockType<V> + Default {
    /// Get the block from the `relative state id`.
    ///
    /// ---
    ///
    /// The relative state id is the state id minus the minimum state id.
    ///
    /// For example, if the minimum state id is `3` and the state id is `4`,
    /// the relative state id is `4 - 3 = 1`.
    #[must_use]
    #[inline]
    fn from_relative_state(rel: u32) -> Option<Self> {
        match rel {
            0 => Some(Self::default()),
            _ => None,
        }
    }

    /// Get the `relative state id` from the block.
    ///
    /// ---
    ///
    /// The relative state id is the state id minus the minimum state id.
    ///
    /// For example, if the minimum state id is `6` and the state id is `6`,
    /// the relative state id is `6 - 6 = 0`.
    fn to_relative_state(&self) -> u32 { 0 }
}

/// A trait that registers blocks inside the block registry.
pub trait BlockRegistration: Version {
    /// Register the default blocks.
    fn register_default(registry: &mut InnerRegistry<Self>);
}
