use std::{any::Any, fmt::Debug};

use compact_str::CompactString;
use downcast_rs::Downcast;
use froglight_protocol::{common::ResourceKey, traits::Version};

use super::BlockStorage;

/// A block for a specific [`Version`].
pub trait BlockType<V>
where
    Self: 'static + Any + Downcast + Debug + Send + Sync,
    V: Version,
{
    /// The block's [`ResourceKey`].
    #[must_use]
    fn to_key(&self) -> ResourceKey;
    /// The block's language key.
    #[must_use]
    fn to_lang(&self) -> CompactString;

    /// Returns `true` if the block is air.
    #[must_use]
    fn is_air(&self) -> bool { false }
    /// Returns `true` if the block is opaque.
    #[must_use]
    fn is_opaque(&self) -> bool { true }
    /// Returns `true` if the block is collidable.
    #[must_use]
    fn is_collidable(&self) -> bool { true }
}

/// An extension trait for [`BlockType`].
pub trait BlockExt<V>
where
    Self: Sized + BlockType<V>,
    V: Version,
{
    /// The total number of block states.
    ///
    /// This is equal to all of the attribute's states multiplied together.
    const BLOCK_STATES: u32 = 1u32;

    /// Create a new block from it's default state.
    #[must_use]
    fn default_state() -> Self;

    /// Get a blockstate from it's relative ID.
    #[must_use]
    fn from_relative_id(id: u32) -> Option<Self> {
        if id == 0 {
            Some(Self::default_state())
        } else {
            None
        }
    }
    /// Get a blockstate's relative ID.
    #[must_use]
    fn to_relative_id(&self) -> u32 { 0 }
}

/// A block attribute for a specific [`Version`].
pub trait BlockAttribute<V>
where
    V: Version,
{
    /// A count of the attribute's states.
    const STATES: u32;
}

/// A block state resolver for a specific [`Version`].
pub trait BlockStateResolver<V>
where
    Self: 'static + Debug + Send + Sync,
    V: Version,
{
    /// The result of the resolver.
    type Result;

    /// Resolve a block state from it's state id.
    #[must_use]
    fn resolve(state_id: u32, storage: &BlockStorage<V>) -> Self::Result;

    /// Register all default blocks for this resolver.
    fn register_defaults(storage: &mut BlockStorage<V>);
}
