use bevy_log::{trace, warn};
use froglight_protocol::traits::Version;
use rangemap::RangeMap;

use crate::blocks::BlockType;

/// The inner registry for the block registry.
#[derive(Debug, Default)]
pub struct InnerRegistry<V: Version> {
    /// The list of blocks in the registry.
    pub(crate) blocks: Vec<Box<dyn BlockType<V>>>,

    /// A map of block states to their index in the block list.
    pub(crate) block_states: RangeMap<u32, usize>,
}

impl<V: Version> InnerRegistry<V> {
    /// Creates a new empty registry.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get the block index for the given state.
    #[must_use]
    pub fn get_block_index(&self, state: u32) -> Option<usize> {
        if let Some(index) = self.block_states.get(&state) {
            Some(*index)
        } else {
            warn!("Attempted to get block id for unregistered state: {state}");
            None
        }
    }

    /// Get the block state for the given block index.
    #[must_use]
    pub fn get_block_from_index(&self, index: usize) -> Option<&dyn BlockType<V>> {
        if let Some(block) = self.blocks.get(index) {
            Some(block.as_ref())
        } else {
            warn!("Attempted to get block with unknown index: {index}");
            None
        }
    }

    /// Get the block for the given state.
    #[must_use]
    pub fn get_block(&self, state: u32) -> Option<&dyn BlockType<V>> {
        self.get_block_index(state).and_then(|i| self.get_block_from_index(i))
    }

    /// Register a block in the registry.
    ///
    /// # Note
    /// It does not matter what state the block is in when it is registered.
    pub fn register_block(&mut self, block: impl BlockType<V>) -> &mut Self {
        trace!("Registering {:?} block: {}", V::default(), block.resource_key());

        let index = self.blocks.len();
        let states = block.states();
        self.blocks.push(Box::new(block));

        if let Some((range, _)) = self.block_states.last_range_value() {
            // Add the new block after the last block.
            self.block_states.insert(range.end..range.end + states, index);
        } else {
            // Add the new block at the start of the map.
            self.block_states.insert(0..states, index);
        }

        self
    }
}
