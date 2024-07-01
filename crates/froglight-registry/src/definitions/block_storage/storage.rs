#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;
use std::{any::TypeId, ops::Range};

use froglight_protocol::traits::Version;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
use rangemap::RangeMap;

use super::{BlockExt, BlockStateResolver, BlockType, VanillaResolver};

/// Storage for all blocks of a specific [`Version`].
#[derive(Debug, Default)]
pub struct BlockStorage<V: Version> {
    /// All of the blocks for a specific [`Version`].
    pub(crate) dyn_storage: Vec<Box<dyn BlockType>>,
    /// A map of block state ranges to their index in `dyn_storage`.
    pub(crate) range_map: RangeMap<u32, usize>,
    /// A map of block type ids to their block state ranges.
    pub(crate) type_map: HashMap<TypeId, Range<u32>>,

    _v: std::marker::PhantomData<V>,
}

/// Implementations for creating a new [`BlockStorage`] and registering blocks.
impl<V: Version> BlockStorage<V> {
    /// The default capacity for a [`BlockStorage`] when using
    /// [`BlockStorage::new`].
    pub const DEFAULT_CAPACITY: usize = 1024 + 128;

    /// Create a new [`BlockStorage`] with all
    /// [`vanilla blocks`](VanillaResolver) registered.
    #[must_use]
    pub fn new() -> Self
    where
        VanillaResolver: BlockStateResolver<V>,
    {
        let mut storage = Self::with_capacity(Self::DEFAULT_CAPACITY);
        storage.register_resolver::<VanillaResolver>();
        storage
    }

    /// Create a new [`BlockStorage`] with a specific capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dyn_storage: Vec::with_capacity(capacity),
            type_map: HashMap::with_capacity(capacity),
            ..Self::new_empty()
        }
    }

    /// Create a new empty [`BlockStorage`].
    #[must_use]
    #[cfg(feature = "hashbrown")]
    pub const fn new_empty() -> Self {
        Self {
            dyn_storage: Vec::new(),
            range_map: RangeMap::new(),
            type_map: HashMap::with_hasher(hashbrown::hash_map::DefaultHashBuilder::new()),
            _v: std::marker::PhantomData,
        }
    }

    /// Create a new empty [`BlockStorage`].
    #[must_use]
    #[cfg(not(feature = "hashbrown"))]
    pub fn new_empty() -> Self {
        Self {
            dyn_storage: Vec::new(),
            range_map: RangeMap::new(),
            type_map: HashMap::new(),
            _v: std::marker::PhantomData,
        }
    }

    /// Register all blocks for a specific [`BlockStateResolver`].
    pub fn register_resolver<Res: BlockStateResolver<V>>(&mut self) -> &mut Self {
        Res::register_blocks(self);
        self
    }

    /// Register a new block type with the [`BlockStorage`].
    ///
    /// This is usually called by a [`BlockStateResolver`]
    /// when using [`BlockStorage::register_resolver`].
    pub fn register<Block: BlockExt<V>>(&mut self) -> &mut Self {
        // Create a new default block.
        let default = Block::default_state();

        // Get the next available storage index and insert the block.
        let storage_index = self.dyn_storage.len();
        self.dyn_storage.push(Box::new(default));

        // Calculate the block state range.
        let (last_range, _) = self.range_map.last_range_value().unwrap_or((&(0..0), &0));
        let new_range = last_range.end..(last_range.end + Block::BLOCK_STATES);

        // Insert the block state range and type id.
        self.range_map.insert(new_range.clone(), storage_index);
        self.type_map.insert(TypeId::of::<Block>(), new_range);

        self
    }
}

/// Implementations for converting between blocks and block state ids.
impl<V: Version> BlockStorage<V> {
    /// Get the (exclusive) range of `block states` for a block, when you don't
    /// know the block type.
    ///
    ///  # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     definitions::{BlockExt, BlockStorage},
    ///     registries::{attributes::SnowyBooleanAttribute, blocks::GrassBlock},
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // Here we know the block type, but you can use any `dyn BlockType`.
    /// let grass_range =
    ///     storage.blockstate_range(&GrassBlock { snowy: SnowyBooleanAttribute(true) }).unwrap();
    /// // Grass has 2 block states, `8` and `9`.
    /// assert_eq!(grass_range, &(8..10));
    /// ```
    #[must_use]
    pub fn blockstate_range(&self, block: &dyn BlockType) -> Option<&Range<u32>> {
        self.type_map.get(&block.type_id())
    }

    /// Get the range of `block states` for a `block state id`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::definitions::BlockStorage;
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // Stone has 1 block state, `1`.
    /// let stone_range = storage.blockstate_range_of(1).unwrap();
    /// assert_eq!(stone_range, &(1..2));
    ///
    /// // Grass has 2 block states, `8` and `9`.
    /// let grass_range = storage.blockstate_range_of(8).unwrap();
    /// assert_eq!(grass_range, storage.blockstate_range_of(9).unwrap());
    /// assert_eq!(grass_range, &(8..10));
    /// ```
    #[must_use]
    pub fn blockstate_range_of(&self, blockstate_id: u32) -> Option<&Range<u32>> {
        self.range_map.get_key_value(&blockstate_id).map(|(range, _)| range)
    }

    /// Get the `block id` of a dyn block.
    ///
    /// This equivalent to the order in which the blocks were registered.
    ///
    /// # Note
    /// This ***is not*** the same as the `block state id`!
    ///
    /// You likely want to use [`BlockStorage::blockstate_id_of`] instead.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     definitions::BlockStorage,
    ///     registries::{
    ///         attributes::SnowyBooleanAttribute,
    ///         blocks::{Air, GrassBlock, Stone},
    ///     },
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // Air was registered first, so it has an id of `0`.
    /// let air_id = storage.block_id(&Air).unwrap();
    /// assert_eq!(air_id, 0);
    ///
    /// // Stone was registered second, so it has an id of `1`.
    /// let stone_id = storage.block_id(&Stone).unwrap();
    /// assert_eq!(stone_id, 1);
    ///
    /// // Grass was registered ninth, so it has an id of `8`.
    /// let grass_id = storage.block_id(&GrassBlock { snowy: SnowyBooleanAttribute(true) }).unwrap();
    /// // Both variants of grass have the same `block id`.
    /// assert_eq!(
    ///     grass_id,
    ///     storage.block_id(&GrassBlock { snowy: SnowyBooleanAttribute(false) }).unwrap()
    /// );
    /// assert_eq!(grass_id, 8);
    /// ```
    #[must_use]
    pub fn block_id(&self, block: &dyn BlockType) -> Option<usize> {
        self.blockstate_range(block).and_then(|range| self.range_map.get(&range.start)).copied()
    }

    /// Get the `block id` of a block from it's `block state id`.
    ///
    /// This equivalent to the order in which the blocks were registered.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::definitions::BlockStorage;
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // Air was registered first, so it has a `block id` of `0`.
    /// assert_eq!(storage.block_id_of(0), Some(0));
    ///
    /// // Stone was registered second, so it has a `block id` of `1`.
    /// assert_eq!(storage.block_id_of(1), Some(1));
    ///
    /// // Grass was registered ninth, so it has a `block id` of `8`.
    /// assert_eq!(storage.block_id_of(8), Some(8));
    /// assert_eq!(storage.block_id_of(9), Some(8));
    /// ```
    #[must_use]
    pub fn block_id_of(&self, blockstate_id: u32) -> Option<usize> {
        self.range_map.get(&blockstate_id).copied()
    }

    /// Get the `block state id` of a block.
    ///
    /// This is the reverse of [`BlockStorage::resolve_blockstate`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     definitions::{BlockExt, BlockStorage},
    ///     registries::{attributes::SnowyBooleanAttribute, blocks::GrassBlock},
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // The first variant of grass has `SnowyBooleanAttribute(true)`.
    /// let grass_snowy = storage.blockstate_id_of(&GrassBlock { snowy: SnowyBooleanAttribute(true) });
    /// assert_eq!(grass_snowy, Some(8));
    ///
    /// // The second variant of grass has `SnowyBooleanAttribute(false)`.
    /// let grass_normal =
    ///     storage.blockstate_id_of(&GrassBlock { snowy: SnowyBooleanAttribute(false) });
    /// assert_eq!(grass_normal, Some(9));
    /// ```
    #[must_use]
    pub fn blockstate_id_of(&self, block: &impl BlockExt<V>) -> Option<u32> {
        block.to_blockstate_id(self)
    }

    /// Returns the default block for a `block state id`.
    ///
    /// # Note
    /// This is only useful if you don't know the block type,
    /// and the properties of the block are not needed
    /// or will not change between different block states.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::definitions::BlockStorage;
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // Pretend we don't know the block type.
    /// let grass_snowy = storage.default_blockstate(8).unwrap();
    /// let grass_normal = storage.default_blockstate(9).unwrap();
    ///
    /// // The block key is the same for both block states.
    /// assert_eq!(grass_snowy.to_key(), "minecraft:grass_block");
    /// assert_eq!(grass_normal.to_key(), "minecraft:grass_block");
    ///
    /// // Both block states are not `air`.
    /// assert!(!grass_snowy.is_air());
    /// assert!(!grass_normal.is_air());
    ///
    /// // However, specific properties that should be different are the same.
    /// // This is because we are using the *default* (not `snowy`) block state.
    /// // assert_eq!(**grass_snowy.is_snowy(), Some(false));
    /// // assert_eq!(**grass_normal.is_snowy(), Some(false));
    /// ```
    #[must_use]
    pub fn default_blockstate(&self, blockstate_id: u32) -> Option<&dyn BlockType> {
        self.block_id_of(blockstate_id)
            .and_then(|block_id| self.dyn_storage.get(block_id).map(std::convert::AsRef::as_ref))
    }

    /// Resolve a [`Res::Resolved`] from a `block state id`.
    ///
    /// This is the reverse of [`BlockStorage::blockstate_id_of`].
    ///
    /// # Example
    /// ```rust,no_run
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    /// use froglight_registry::{
    ///     definitions::{BlockStateResolver, BlockStorage, VanillaResolver},
    ///     registries::{attributes::SnowyBooleanAttribute, blocks::GrassBlock},
    /// };
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // The first variant of grass has `SnowyBooleanAttribute(true)`.
    /// let grass_snowy = storage.resolve_blockstate::<VanillaResolver>(8).unwrap();
    /// assert_eq!(grass_snowy, GrassBlock { snowy: SnowyBooleanAttribute(true) }.into());
    ///
    /// // The second variant of grass has `SnowyBooleanAttribute(false)`.
    /// let grass_normal = storage.resolve_blockstate::<VanillaResolver>(9).unwrap();
    /// assert_eq!(grass_normal, GrassBlock { snowy: SnowyBooleanAttribute(false) }.into());
    /// ```
    #[must_use]
    pub fn resolve_blockstate<Res: BlockStateResolver<V>>(
        &self,
        blockstate_id: u32,
    ) -> Res::Resolved {
        Res::resolve_state(blockstate_id, self)
    }
}
