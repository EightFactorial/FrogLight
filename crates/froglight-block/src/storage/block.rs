use std::{any::TypeId, ops::Range, sync::Arc};

use derive_more::derive::Deref;
use downcast_rs::Downcast;
use froglight_common::Version;
use parking_lot::RwLock;
use rangemap::RangeMap;

use super::{GlobalBlockId, RelativeBlockState};
use crate::{
    block::{BlockType, BlockTypeExt, UntypedBlock},
    storage::BlockAttributes,
};

/// A thread-safe dynamic storage for block types.
///
/// Allows for the registration and retrieval of block types at runtime.
#[derive(Default, Clone, Deref)]
pub struct AppBlockStorage<V: Version>(Arc<RwLock<BlockStorage<V>>>);

impl<V: Version> AppBlockStorage<V> {
    /// Create a new [`AppBlockStorage`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Create a new [`AppBlockStorage`] from a [`BlockStorage`].
    #[must_use]
    pub fn from_storage(storage: BlockStorage<V>) -> Self { Self(Arc::new(RwLock::new(storage))) }
}

/// A dynamic storage for block types.
///
/// Allows for the registration and retrieval of block types at runtime.
pub struct BlockStorage<V: Version> {
    traits: RangeMap<u32, BlockWrapper<V>>,
    #[cfg(feature = "bevy")]
    types: bevy_utils::TypeIdMap<u32>,
    #[cfg(not(feature = "bevy"))]
    types: hashbrown::HashMap<TypeId, u32>,
}

impl<V: Version> Default for BlockStorage<V> {
    fn default() -> Self { Self::new() }
}

impl<V: Version> BlockStorage<V> {
    /// Create a new [`BlockStorage`] with the default block types registered.
    #[must_use]
    #[expect(unused_mut, clippy::let_and_return)]
    pub fn new() -> Self {
        let mut storage = Self::new_empty();
        storage
    }

    /// Create a new [`BlockStorage`] with no registered block types.
    #[must_use]
    #[expect(clippy::default_trait_access)]
    pub fn new_empty() -> Self { Self { traits: RangeMap::new(), types: Default::default() } }

    /// Get the [`UntypedBlock`] for the given block.
    #[must_use]
    pub fn get_untyped(&self, block: GlobalBlockId) -> Option<UntypedBlock<V>> {
        let (range, wrapper) = self.traits.get_key_value(&block)?;
        Some(UntypedBlock::new(RelativeBlockState::from(range.start - *block), *wrapper))
    }

    /// Get the [`GlobalBlockId`] for the given block.
    #[must_use]
    pub fn get_global(&self, block: impl Into<UntypedBlock<V>>) -> Option<GlobalBlockId> {
        let block: UntypedBlock<V> = block.into();
        let start = self.types.get(&Downcast::as_any(block.wrapper()).type_id())?;
        Some(GlobalBlockId::new_unchecked(*start + u32::from(**block.state())))
    }

    /// Register a block type with the storage.
    #[expect(clippy::cast_possible_truncation)]
    pub fn register<B: BlockTypeExt<V>>(&mut self) {
        let range = self.traits.last_range_value().map_or_else(
            || Range { start: 0, end: B::Attributes::COUNT as u32 },
            |(r, _)| Range { start: r.end + 1, end: r.end + B::Attributes::COUNT as u32 },
        );
        self.types.insert(TypeId::of::<B>(), range.start);
        self.traits.insert(range, BlockWrapper::new(B::as_static()));
    }
}

/// A wrapper around a [`&'static dyn BlockType`](BlockType)
/// that implements [`PartialEq`] and [`Eq`].
#[derive(Clone, Copy, Deref)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(PartialEq))]
pub(crate) struct BlockWrapper<V: Version>(&'static dyn BlockType<V>);

impl<V: Version> BlockWrapper<V> {
    /// Create a new [`BlockWrapper`] from the given block type.
    #[must_use]
    pub(crate) fn new(block: &'static dyn BlockType<V>) -> Self { Self(block) }
}

impl<V: Version> Eq for BlockWrapper<V> {}
impl<V: Version> PartialEq for BlockWrapper<V> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other) && (self.0.type_id() == other.0.type_id())
    }
}
