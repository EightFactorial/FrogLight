use std::{any::TypeId, ops::Range};

use froglight_protocol::traits::Version;
use hashbrown::HashMap;
use rangemap::RangeMap;

mod traits;
pub use traits::*;

/// Storage for all blocks of a specific [`Version`].
#[derive(Debug, Default)]
pub struct BlockStorage<V: Version> {
    /// All of the blocks for a specific [`Version`].
    pub(crate) dyn_storage: Vec<Box<dyn BlockType<V>>>,

    /// A map of block state ranges to their index in `dyn_storage`.
    pub(crate) range_map: RangeMap<u32, usize>,

    /// A map of block type ids to their block state ranges.
    pub(crate) type_map: HashMap<TypeId, Range<u32>>,
}

/// [`BlockStateResolver`] for vanilla blocks.
///
/// To be used with [`BlockStorage::get_block`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VanillaResolver;

impl<V: Version> BlockStorage<V> {
    /// Create a new empty [`BlockStorage`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get a block's state id range.
    #[must_use]
    pub fn range_of<Block: BlockType<V>>(&self) -> Option<&Range<u32>> {
        self.type_map.get(&TypeId::of::<Block>())
    }

    /// Get a block's relative state id from it's type and state id.
    ///
    /// This will return `None` if the [`BlockType`] is not registered or
    /// the state id is out of range.
    #[must_use]
    pub fn relative_state_of<Block: BlockType<V>>(&self, state_id: u32) -> Option<u32> {
        let range = self.range_of::<Block>()?;
        if range.contains(&state_id) {
            state_id.checked_sub(range.start)
        } else {
            None
        }
    }

    /// Get a block's default state as a trait object.
    ///
    /// ---
    ///
    /// ### Note
    /// This returns a reference to the **default block**, who's properties
    /// may not match the actual block state.
    ///
    /// This is useful if you only need default block properties, or if you
    /// are sure the property is the same for all block states.
    ///
    /// If you want to get the full block state, use
    /// [`BlockStorage::get_block`].
    #[must_use]
    pub fn get_default_dyn(&self, state_id: u32) -> Option<&dyn BlockType<V>> {
        let storage_index = self.range_map.get(&state_id)?;
        self.dyn_storage.get(*storage_index).map(AsRef::as_ref)
    }

    /// Get a block from it's state id.
    ///
    /// See [`BlockStateResolver`] and [`VanillaResolver`] for more information.
    ///
    /// ---
    ///
    /// ### Note
    /// This will resolve the full block, including all properties.
    ///
    /// If you only need the default block properties, use
    /// the much faster [`BlockStorage::get_default_dyn`].
    #[must_use]
    pub fn get_block<Res: BlockStateResolver<V>>(&self, state_id: u32) -> Res::Result {
        Res::resolve(state_id, self)
    }

    /// Register a new block type with the [`BlockStorage`].
    pub fn register<Block: Default + BlockExt<V>>(&mut self) -> &mut Self {
        // Create a new default block.
        let default = Block::default();

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
