use std::{any::TypeId, ops::Range};

use bevy_utils::{NoOpHash, TypeIdMap};
use froglight_protocol::traits::Version;
use rangemap::RangeMap;

use crate::{BlockResolver, BlockState, BlockStateExt, VanillaResolver};

#[cfg(feature = "bevy")]
mod plugin;
#[cfg(feature = "bevy")]
pub use plugin::BlockPlugin;

#[cfg(feature = "bevy")]
mod reflect;
#[cfg(feature = "bevy")]
pub use reflect::{BlockBuilder, ReflectBlockBuilder};

#[cfg(feature = "bevy")]
mod resource;
#[cfg(feature = "bevy")]
pub use resource::BlockStorageArc;

#[cfg(feature = "bevy")]
mod vanilla;
#[cfg(feature = "bevy")]
pub use vanilla::VanillaBuilder;

#[cfg(test)]
mod test;

/// A storage container for blocks.
pub struct BlockStorage<V: Version> {
    type_map: TypeIdMap<Box<dyn BlockState<V>>>,
    type_range: RangeMap<u32, TypeId>,
}

impl<V: Version> BlockStorage<V> {
    /// Create a new [`BlockStorage`].
    #[inline]
    #[must_use]
    pub const fn new_empty() -> Self {
        Self { type_map: TypeIdMap::with_hasher(NoOpHash), type_range: RangeMap::new() }
    }

    /// Get the [`TypeId`] of the block with the given ID.
    #[must_use]
    pub fn get_type(&self, block_id: u32) -> Option<TypeId> {
        self.type_range.get(&block_id).copied()
    }

    /// Get the block ID of a given block.
    #[must_use]
    pub fn get_block_id<B: BlockStateExt<V>>(&self, block: &B) -> Option<u32> {
        if let Some((type_range, _)) =
            self.type_range.iter().find(|(_, type_id)| **type_id == TypeId::of::<B>())
        {
            Some(type_range.start + u32::from(block.to_relative()))
        } else {
            None
        }
    }

    /// Get the block with the given ID.
    ///
    /// Requires knowing the block type ahead of time.
    #[must_use]
    pub fn get_known_block<B: BlockStateExt<V>>(&self, block_id: u32) -> Option<B> {
        let (range, _) = self.type_range.get_key_value(&block_id)?;
        B::from_relative(u16::try_from(block_id - range.start).ok()?)
    }

    /// Get the block with the given ID using a [`BlockResolver`].
    ///
    /// # Note
    /// This is a wrapper around [`BlockResolver::resolve`].
    #[inline]
    #[must_use]
    pub fn get_block<R: BlockResolver<V>>(&self, block_id: u32) -> R::Output {
        R::resolve(block_id, self)
    }
}

impl<V: Version> BlockStorage<V>
where
    VanillaResolver: BlockResolver<V>,
{
    /// Get the vanilla block with the given ID.
    ///
    /// # Note
    /// This is a wrapper around [`BlockResolver::resolve`]
    /// using [`VanillaResolver`] as the resolver.
    #[inline]
    #[must_use]
    pub fn get_vanilla(&self, block_id: u32) -> <VanillaResolver as BlockResolver<V>>::Output {
        VanillaResolver::resolve(block_id, self)
    }
}

impl<V: Version> BlockStorage<V> {
    /// Get the default [`BlockState`] for the block with the given ID.
    #[must_use]
    pub fn get_stored_default(&self, block_id: u32) -> Option<&dyn BlockState<V>> {
        self.type_range.get(&block_id).and_then(|type_id| self.get_stored_default_dyn(*type_id))
    }

    /// Get the default [`BlockState`] for the block with the given [`TypeId`].
    #[must_use]
    pub fn get_stored_default_dyn(&self, type_id: TypeId) -> Option<&dyn BlockState<V>> {
        self.type_map.get(&type_id).map(|boxed| &**boxed)
    }

    /// Get the default [`BlockState`] for the block with the given type.
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn get_stored_default_type<B: BlockState<V>>(&self) -> Option<&B> {
        self.get_stored_default_dyn(TypeId::of::<B>())
            .map(|boxed| boxed.as_any().downcast_ref().expect("Block TypeId mismatch"))
    }
}

impl<V: Version> BlockStorage<V> {
    /// Register a new block type with the [`BlockStorage`].
    #[expect(clippy::missing_panics_doc)]
    pub fn register<B: BlockStateExt<V>>(&mut self) {
        let (last_range, _last_type) =
            self.type_range.last_range_value().unwrap_or((&(0..0), &TypeId::of::<B>()));

        // Get the range of state ids for the new block type
        let states = u32::try_from(B::STATE_COUNT).unwrap();
        let new_range = Range { start: last_range.end, end: last_range.end + states };

        // Insert the new block type into the storage
        self.type_map.insert(TypeId::of::<B>(), Box::new(B::DEFAULT));
        self.type_range.insert(new_range, TypeId::of::<B>());
    }
}
