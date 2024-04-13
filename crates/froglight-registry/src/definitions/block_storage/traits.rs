use std::{any::Any, fmt::Debug};

use compact_str::CompactString;
use froglight_protocol::{common::ResourceKey, traits::Version};

use super::BlockStorage;

/// A block for a specific [`Version`].
pub trait BlockType<V>
where
    Self: 'static + Any + Debug + Send + Sync,
    V: Version,
{
    /// The block's [`ResourceKey`].
    fn to_key(&self) -> ResourceKey;
    /// The block's language key.
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
    const BLOCK_STATES: u32;

    /// Get a blockstate from it's relative ID.
    fn from_relative_id(id: u32) -> Option<Self>;
    /// Get a blockstate's relative ID.
    fn to_relative_id(&self) -> u32 { 0 }
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
    fn resolve(state_id: u32, storage: &BlockStorage<V>) -> Self::Result;
}
