use std::any::TypeId;

use bevy_log::{trace, warn};
use froglight_protocol::traits::Version;
use hashbrown::HashMap;
use rangemap::RangeMap;

use crate::blocks::BlockType;

/// The inner registry for the block registry.
#[derive(Debug, Default)]
pub struct InnerRegistry<V: Version> {
    /// The list of blocks in the registry.
    pub(crate) blocks: Vec<Box<dyn BlockType<V>>>,

    /// A map of block states to their index in the block list.
    pub(crate) block_states: RangeMap<u32, usize>,

    /// A map of block type ids to their index in the block list.
    pub(crate) block_info: HashMap<TypeId, u32>,
}

impl<V: Version> InnerRegistry<V> {
    /// Creates a new empty registry.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get the block state id range for the given state.
    #[must_use]
    pub fn state_range(&self, state: u32) -> Option<&std::ops::Range<u32>> {
        self.block_states.get_key_value(&state).map(|(range, _)| range)
    }

    /// Get the relative state id for the given state.
    ///
    /// This is shorthand for `state - range.start`.
    #[must_use]
    pub fn relative_state(&self, state: u32) -> Option<u32> {
        self.block_states.get_key_value(&state).map(|(range, _)| state - range.start)
    }

    /// Get the block state id range for the given block type.
    #[must_use]
    pub fn type_range<T: BlockType<V>>(&self) -> Option<&std::ops::Range<u32>> {
        self.block_info.get(&TypeId::of::<T>()).and_then(|index| self.state_range(*index))
    }

    /// Get the block state for the given block index.
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn get_block_type(&self, index: usize) -> Option<&dyn BlockType<V>> {
        if let Some(block) = self.blocks.get(index) {
            Some(block.as_ref())
        } else {
            warn!("Attempted to get block with unknown index: {index}");
            None
        }
    }

    /// Get the block index for the given state.
    #[must_use]
    pub fn block_index(&self, state: u32) -> Option<usize> {
        if let Some(index) = self.block_states.get(&state) {
            Some(*index)
        } else {
            warn!("Attempted to get block id for unregistered state: {state}");
            None
        }
    }

    /// Get the block for the given state.
    ///
    /// # Note
    /// This will always return a trait object pointing towards the block's
    /// default state.
    ///
    /// This is useful for when you're sure that the block
    /// you want doesn't change if the state changes, like the
    /// [`BlockType<V>::resource_key`] and [`BlockType<V>::is_air`] methods.
    ///
    /// If you need information about a block with a specific state, use
    /// [`BlockEnum::from_dyn`](`crate::blocks::BlockEnum::from_dyn`).
    #[must_use]
    pub fn get_block(&self, state: u32) -> Option<&dyn BlockType<V>> {
        self.block_index(state).and_then(|i| self.get_block_type(i))
    }

    /// Register a block in the registry.
    pub fn register_block<B: BlockType<V> + Default>(&mut self) -> &mut Self {
        let block = B::default();

        #[cfg(debug_assertions)]
        {
            trace!("Registering {:?} block: {}", V::default(), block.resource_key());
        }

        // Get the block information
        let index = self.blocks.len();
        let states = block.states();

        // Add the block to the list of blocks.
        self.blocks.push(Box::new(block));

        if let Some((range, _)) = self.block_states.last_range_value() {
            // Add the block to the block info map.
            self.block_info.insert(TypeId::of::<B>(), range.end);

            // Add the new block after the last block.
            self.block_states.insert(range.end..range.end + states, index);
        } else {
            // Add the block to the block info map.
            self.block_info.insert(TypeId::of::<B>(), 0);

            // Add the new block at the start of the map.
            self.block_states.insert(0..states, index);
        }

        self
    }
}
