use std::{any::TypeId, ops::Range};

use froglight_protocol::traits::Version;
use hashbrown::HashMap;
use rangemap::RangeMap;

use crate::blocks::{block_list::BlockEnum, traits::BlockResolution, BlockType};

/// The inner registry for the block registry.
///
/// # Example
/// ```rust
/// use froglight_protocol::versions::v1_20_0::V1_20_0;
/// use froglight_world::blocks::{
///     block_list::{BlockAir, BlockCobblestone, BlockEnum, BlockStone},
///     BlockRegistry,
/// };
///
/// // Create a new block registry from the world.
/// let registry = BlockRegistry::<V1_20_0>::new_default();
/// let registry = registry.read();
///
/// // Note: Ranges are exclusive, so `0..1` only contains the block state id `0`.
///
/// // Get the block state id range for air
/// let block_range = registry.range_of::<BlockAir>();
/// assert_eq!(block_range, Some(&(0..1)));
///
/// // Get the block from a block state id
/// let block_state = registry.get_block(0).unwrap();
/// assert_eq!(block_state, BlockEnum::Air(BlockAir));
///
/// let block_state = registry.get_block(1).unwrap();
/// assert_eq!(block_state, BlockEnum::Stone(BlockStone));
///
/// let block_state = registry.get_block(14).unwrap();
/// assert_eq!(block_state, BlockEnum::Cobblestone(BlockCobblestone));
/// ```
#[derive(Debug, Default)]
pub struct InnerBlockRegistry<V: Version> {
    /// A collection of blocks inside the registry.
    pub(crate) dyn_blocks: Vec<Box<dyn BlockType<V>>>,

    /// A map of block state ids to block indices.
    pub(crate) range_map: RangeMap<u32, usize>,

    /// A map of block type ids to block id ranges.
    pub(crate) type_map: HashMap<TypeId, Range<u32>>,
}

impl<V: Version> InnerBlockRegistry<V> {
    /// Creates a new empty registry.
    #[must_use]
    #[inline]
    pub fn new() -> Self { Self::default() }

    /// Gets the block state id range for a block type.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_20_0::V1_20_0;
    /// use froglight_world::blocks::{block_list::BlockAir, BlockRegistry};
    ///
    /// let registry = BlockRegistry::<V1_20_0>::new_default();
    /// let registry = registry.read();
    ///
    /// let range = registry.range_of::<BlockAir>();
    /// assert_eq!(range, Some(&(0..1)));
    /// ```
    #[must_use]
    #[inline]
    pub fn range_of<T: BlockType<V>>(&self) -> Option<&Range<u32>> {
        self.type_map.get(&TypeId::of::<T>())
    }

    /// Gets the relative state id from the block type and block state id.
    ///
    /// This is useful if you already know what block type the block state id is
    /// from.
    ///
    /// # Example
    /// ```rust
    /// use std::ops::Range;
    ///
    /// use froglight_protocol::versions::v1_20_0::V1_20_0;
    /// use froglight_world::blocks::{
    ///     attributes::SnowyAttribute, block_list::BlockGrassBlock, BlockExt, BlockRegistry,
    /// };
    ///
    /// let registry = BlockRegistry::<V1_20_0>::new_default();
    /// let registry = registry.read();
    ///
    /// // Ranges are exclusive, so the range `X..X+2` contains the block state ids `X` and `X+1`.
    /// let Range { start, end } = registry.range_of::<BlockGrassBlock>().unwrap();
    ///
    /// // The first block state is the grass block without snow.
    /// let state_zero = registry.relative_state_of::<BlockGrassBlock>(*start).unwrap();
    /// let blockstate = BlockGrassBlock::from_relative_state(state_zero);
    /// assert_eq!(blockstate, Some(BlockGrassBlock { snowy: SnowyAttribute(false) }));
    ///
    /// // The second block state is the grass block with snow.
    /// let state_one = registry.relative_state_of::<BlockGrassBlock>(end - 1).unwrap();
    /// let blockstate = BlockGrassBlock::from_relative_state(state_one);
    /// assert_eq!(blockstate, Some(BlockGrassBlock { snowy: SnowyAttribute(true) }));
    /// ```
    #[must_use]
    #[inline]
    pub fn relative_state_of<T: BlockType<V>>(&self, state: u32) -> Option<u32> {
        let range = self.range_of::<T>()?;
        state.checked_sub(range.start)
    }

    /// Gets a `dyn block` from the registry.
    ///
    /// # Note
    /// This returns a reference to the default block and it's properties
    /// likely do not match the actual block state.
    ///
    /// This is useful if you want to get the default block properties,
    /// or if you want to get properties you are sure are the same for all block
    /// states.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_20_0::V1_20_0;
    /// use froglight_world::blocks::BlockRegistry;
    ///
    /// let registry = BlockRegistry::<V1_20_0>::new_default();
    /// let registry = registry.read();
    ///
    /// // `BlockAir` is the first block in the registry, so the block state id is `0`.
    /// let block = registry.get_dyn(0).unwrap();
    ///
    /// // The `ResourceKey` for air will always be the same.
    /// assert_eq!(block.resource_key(), "minecraft:air");
    /// ```
    #[must_use]
    #[inline]
    pub fn get_dyn(&self, state: u32) -> Option<&dyn BlockType<V>> {
        let block_index = self.range_map.get(&state)?;
        self.dyn_blocks.get(*block_index).map(AsRef::as_ref)
    }

    /// Gets a block from the registry.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_20_0::V1_20_0;
    /// use froglight_world::blocks::{
    ///     block_list::{BlockAir, BlockEnum, BlockStone},
    ///     BlockRegistry,
    /// };
    ///
    /// let registry = BlockRegistry::<V1_20_0>::new_default();
    /// let registry = registry.read();
    ///
    /// // `BlockAir` is the first block in the registry, so the block state id is `0`.
    /// let block = registry.get_block(0).unwrap();
    /// assert_eq!(block, BlockEnum::Air(BlockAir));
    ///
    /// // `BlockStone` is the second block in the registry, so the block state id is `1`.
    /// let block = registry.get_block(1).unwrap();
    /// assert_eq!(block, BlockEnum::Stone(BlockStone));
    /// ```
    #[must_use]
    #[inline]
    pub fn get_block(&self, state: u32) -> Option<BlockEnum>
    where
        V: BlockResolution,
    {
        V::get_block(state, self)
    }

    /// Register a block in the registry.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::versions::v1_20_0::V1_20_0;
    /// use froglight_world::blocks::{
    ///     block_list::{BlockAir, BlockStone},
    ///     InnerBlockRegistry,
    /// };
    ///
    /// // Create an empty inner registry
    /// let mut registry = InnerBlockRegistry::<V1_20_0>::new();
    ///
    /// // Register the stone block
    /// registry.register_block::<BlockStone>();
    ///
    /// // Get the block index for the stone block
    /// let block_range = registry.range_of::<BlockStone>().unwrap();
    ///
    /// // Because `BlockStone` is the first block in the registry, the range is `0..1`.
    /// assert_eq!(block_range, &(0..1));
    ///
    /// // `BlockAir` is not in the registry, so the range is `None`.
    /// assert_eq!(registry.range_of::<BlockAir>(), None);
    /// ```
    pub fn register_block<B: BlockType<V> + Default>(&mut self) -> &mut Self {
        let block = B::default();
        let states = block.states();

        #[cfg(debug_assertions)]
        bevy_log::trace!("Registering block `{}`", block.resource_key());

        // Insert the block into the dyn_blocks
        let index = self.dyn_blocks.len();
        self.dyn_blocks.push(Box::new(block));

        // Use the last range in the range map to calculate the new range
        let (last_range, _) = self.range_map.last_range_value().unwrap_or((&(0..0), &0));
        let new_range = last_range.end..(last_range.end + states);

        // Insert the block id range into the range map and type map
        self.range_map.insert(new_range.clone(), index);
        self.type_map.insert(TypeId::of::<B>(), new_range);

        self
    }
}
