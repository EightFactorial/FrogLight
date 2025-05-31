//! [`BlockStorage`] and [`AppBlockStorage`]

use alloc::sync::Arc;
use core::{any::TypeId, ops::Range};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
use bevy_platform::{collections::HashMap, hash::NoOpHash};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, From, Into};
use downcast_rs::Downcast;
use froglight_common::{vanilla::Vanilla, version::Version};
use froglight_utils::storage::prelude::*;
use rangemap::RangeMap;

use crate::{
    attribute::BlockAttributes,
    block::{BlockType, BlockTypeExt, UntypedBlock},
    resolve::BlockResolver,
};

/// A dynamic storage for block types.
///
/// Allows for the registration and retrieval of block types at runtime.
#[derive(Clone, AppStorage)]
#[storage(index(ident = "GlobalBlockId", inner = "u32"), bevy = "bevy", reflect = "reflect")]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone))]
pub struct BlockStorage<V: Version> {
    statics: RangeMap<u32, StorageWrapper<dyn BlockType<V>>>,
    storage: HashMap<TypeId, u32, NoOpHash>,
}

impl<V: Version> AppBlockStorage<V> {
    /// Create a new [`AppBlockStorage`] with the [`Vanilla`] types registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: BlockResolver<V> {
        Self::from_storage(BlockStorage::new())
    }
}

impl<V: Version> BlockStorage<V> {
    /// Create a new [`BlockStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: BlockResolver<V> {
        let mut storage = Self::new_empty();
        <Vanilla as BlockResolver<V>>::register(&mut storage);
        storage
    }

    /// Create a new [`BlockStorage`] with no registered block types.
    #[must_use]
    pub const fn new_empty() -> Self {
        Self { statics: RangeMap::new(), storage: HashMap::with_hasher(NoOpHash) }
    }

    /// Get the [`BlockType`] for the given [`GlobalBlockId`].
    ///
    /// Handy for storing many block types and bulk operations.
    ///
    /// Returns `None` if no block with the given id was registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new block storage.
    ///     let storage = BlockStorage::<V1_21_4>::new();
    ///
    ///     // Get the trait with the global id of `0`.
    ///     let block = storage.get_trait(GlobalBlockId::new_unchecked(0)).unwrap();
    ///     assert_eq!(block.identifier(), "minecraft:air");
    ///
    ///     // Get the trait with the global id of `1`.
    ///     let block = storage.get_trait(GlobalBlockId::new_unchecked(1)).unwrap();
    ///     assert_eq!(block.identifier(), "minecraft:stone");
    /// }
    /// ```
    #[must_use]
    pub fn get_trait(&self, block: GlobalBlockId) -> Option<&'static dyn BlockType<V>> {
        self.statics.get(&block).map(|val| val.inner())
    }

    /// Get the [`UntypedBlock`] for the given [`GlobalBlockId`].
    ///
    /// Returns `None` if no block with the given id was registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    /// use froglight_common::vanilla::Vanilla;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new block storage.
    ///     let storage = BlockStorage::<V1_21_4>::new();
    ///
    ///     // Get the block with the global id of `0`.
    ///     let block = storage.get_untyped(GlobalBlockId::new_unchecked(0)).unwrap();
    ///     assert_eq!(block.identifier(), "minecraft:air");
    ///     assert_eq!(
    ///         block.resolve::<Vanilla>(),
    ///         Some(Block::<block::Air, V1_21_4>::default().into())
    ///     );
    ///
    ///     // Get the block with the global id of `1`.
    ///     let block = storage.get_untyped(GlobalBlockId::new_unchecked(1)).unwrap();
    ///     assert_eq!(block.identifier(), "minecraft:stone");
    ///     assert_eq!(
    ///         block.resolve::<Vanilla>(),
    ///         Some(Block::<block::Stone, V1_21_4>::default().into())
    ///     );
    /// }
    /// ```
    #[must_use]
    pub fn get_untyped(&self, block: GlobalBlockId) -> Option<UntypedBlock<V>> {
        self.statics.get_key_value(&block).map(|(range, wrapper)| {
            UntypedBlock::new(RelativeBlockState::from(u32::from(block) - range.start), *wrapper)
        })
    }

    /// Get a typed block for the given block id.
    ///
    /// Returns `None` if no block with the given id was registered,
    /// or the block does not exist in the resolver.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    /// use froglight_common::vanilla::Vanilla;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_block::generated::v1_21_4::VersionBlocks;
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new block storage.
    ///     let storage = BlockStorage::<V1_21_4>::new();
    ///
    ///     // Get the block with the global id of `0`.
    ///     let block = storage.get_typed::<Vanilla>(GlobalBlockId::new_unchecked(0));
    ///     if let Some(VersionBlocks::Air(air)) = block {
    ///         assert_eq!(air.identifier(), "minecraft:air");
    ///     } else {
    ///         panic!("Block was not `Air`!");
    ///     }
    ///
    ///     // Get the block with the global id of `1`.
    ///     let block = storage.get_typed::<Vanilla>(GlobalBlockId::new_unchecked(1));
    ///     if let Some(VersionBlocks::Stone(stone)) = block {
    ///         assert_eq!(stone.identifier(), "minecraft:stone");
    ///     } else {
    ///         panic!("Block was not `Stone`!");
    ///     }
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_typed<R: BlockResolver<V>>(&self, block: GlobalBlockId) -> Option<R::BlockEnum> {
        self.get_untyped(block).and_then(R::resolve)
    }

    /// Get the [`GlobalBlockId`] for the given block.
    ///
    /// Returns `None` if the block was not registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new block storage.
    ///     let storage = BlockStorage::<V1_21_4>::new();
    ///
    ///     // Get the `GlobalBlockId` of `Air`.
    ///     let global_id = storage.get_global(Block::<block::Air, V1_21_4>::default()).unwrap();
    ///     assert_eq!(*global_id, 0u32);
    ///
    ///     // Get the `GlobalBlockId` of `Stone`.
    ///     let global_id = storage.get_global(Block::<block::Stone, V1_21_4>::default()).unwrap();
    ///     assert_eq!(*global_id, 1u32);
    /// }
    /// ```
    #[must_use]
    pub fn get_global(&self, block: impl Into<UntypedBlock<V>>) -> Option<GlobalBlockId> {
        let block: UntypedBlock<V> = block.into();
        self.storage
            .get(&<dyn BlockType<V> as Downcast>::as_any(block.wrapper().inner()).type_id())
            .map(|&start| GlobalBlockId::new_unchecked_u32(start + u32::from(*block.state())))
    }

    /// Register a block type with the storage.
    ///
    /// This is required for converting between global ids and blocks.
    ///
    /// # Note
    /// The order in which blocks are registered is important.
    ///
    /// If a block is registered out of order, all following blocks will have
    /// their global ids shifted incorrectly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_block::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new block storage with the vanilla block types registered.
    ///     let storage = BlockStorage::<V1_21_4>::new();
    ///     let air = Block::<block::Air, V1_21_4>::default();
    ///
    ///     // Since `Air` is already registered, we can get its global id.
    ///     assert_eq!(storage.get_global(air), Some(GlobalBlockId::new_unchecked(0)));
    ///
    ///     // Create a new empty block storage.
    ///     let mut storage = BlockStorage::<V1_21_4>::new_empty();
    ///
    ///     // Since `Air` is not registered, it does not have a global id.
    ///     assert_eq!(storage.get_global(air), None);
    ///
    ///     // Register the `Air` block type, now we can get its global id.
    ///     storage.register::<block::Air>();
    ///     assert_eq!(storage.get_global(air), Some(GlobalBlockId::new_unchecked(0)));
    /// }
    /// ```
    #[expect(clippy::missing_panics_doc)]
    pub fn register<B: BlockTypeExt<V>>(&mut self) {
        let count = u32::try_from(B::Attributes::COUNT).expect("BlockType has too many states!");
        let range = self.statics.last_range_value().map_or_else(
            || Range { start: 0, end: count },
            |(r, _)| Range { start: r.end, end: r.end + count },
        );

        self.storage.insert(TypeId::of::<B>(), range.start);
        self.statics.insert(range, StorageWrapper::new(B::as_static()));
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> Default for AppBlockStorage<V>
where Vanilla: BlockResolver<V>
{
    #[inline]
    fn default() -> Self { Self::new() }
}

impl<V: Version> Default for BlockStorage<V>
where Vanilla: BlockResolver<V>
{
    fn default() -> Self { Self::new() }
}

// -------------------------------------------------------------------------------------------------

/// A block's global state id, relative to the block's first state id.
///
/// # Warning
/// There is no guarantee that the given index is valid or represents the
/// same index between versions. Indices may even change between program
/// runs!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From, Into)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub(crate) struct RelativeBlockState(u16);

impl From<u32> for RelativeBlockState {
    #[cfg(debug_assertions)]
    fn from(id: u32) -> Self { Self(u16::try_from(id).expect("RelativeBlockState is too large!")) }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: u32) -> Self { Self(id as u16) }
}
impl From<RelativeBlockState> for u32 {
    fn from(id: RelativeBlockState) -> Self { u32::from(id.0) }
}

impl From<usize> for RelativeBlockState {
    #[cfg(debug_assertions)]
    fn from(id: usize) -> Self {
        Self(u16::try_from(id).expect("RelativeBlockState is too large!"))
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self { Self(id as u16) }
}
impl From<RelativeBlockState> for usize {
    fn from(id: RelativeBlockState) -> Self { usize::from(id.0) }
}

// -------------------------------------------------------------------------------------------------

impl GlobalBlockId {
    /// Create a new index with the given value.
    ///
    /// # Warning
    /// There is no guarantee that the given index is valid or represents the
    /// same index between versions. Indices may even change between program
    /// runs!
    #[inline]
    #[must_use]
    pub const fn new_unchecked_u32(index: u32) -> Self { Self(index) }
}

impl From<usize> for GlobalBlockId {
    #[cfg(debug_assertions)]
    fn from(id: usize) -> Self { Self(u32::try_from(id).expect("GlobalBlockId is too large!")) }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self { Self(id as u32) }
}

impl From<GlobalBlockId> for usize {
    #[cfg(debug_assertions)]
    fn from(id: GlobalBlockId) -> Self {
        usize::try_from(id.0).expect("GlobalBlockId is too large!")
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    fn from(id: GlobalBlockId) -> Self { id.0 as usize }
}
