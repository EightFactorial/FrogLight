use std::sync::Arc;

use bevy_ecs::system::Resource;
use froglight_protocol::traits::Version;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{BlockStateResolver, BlockStorage, VanillaResolver};

/// A registry which contains all blocks for a specific [`Version`].
///
/// Allows converting between blockstate ids and blocks.
#[derive(Clone, Resource)]
pub struct BlockRegistry<V: Version> {
    storage: Arc<RwLock<BlockStorage<V>>>,
}

impl<V: Version> BlockRegistry<V> {
    /// Create a new [`BlockRegistry`].
    ///
    /// This will contains all [`vanilla blocks`](VanillaResolver).
    #[must_use]
    pub fn new() -> Self
    where
        VanillaResolver: BlockStateResolver<V>,
    {
        Self::default()
    }

    /// Create a new empty [`BlockRegistry`].
    #[must_use]
    pub fn new_empty() -> Self {
        Self { storage: Arc::new(RwLock::new(BlockStorage::<V>::new_empty())) }
    }

    /// Register all default blocks for a specific [`BlockStateResolver`].
    pub fn register_defaults<R: BlockStateResolver<V>>(&mut self) {
        R::register_blocks(&mut self.write());
    }

    /// Get a block from a state id.
    #[must_use]
    pub fn get_block<Res: BlockStateResolver<V>>(&self, state_id: u32) -> Res::Resolved
    where
        Res::Resolved: Copy,
    {
        Res::resolve_state(state_id, &self.read())
    }

    /// Get a block from a state id.
    #[must_use]
    pub fn get_block_clone<Res: BlockStateResolver<V>>(&self, state_id: u32) -> Res::Resolved
    where
        Res::Resolved: Clone,
    {
        Res::resolve_state(state_id, &self.read())
    }

    /// Get a [`RwLockReadGuard`] to the [`BlockStorage`].
    ///
    /// This is useful for reading the block storage.
    ///
    /// ---
    ///
    /// [`Note`](RwLock::read): This may cause a deadlock if the lock is not
    /// released.
    pub fn read(&self) -> RwLockReadGuard<'_, BlockStorage<V>> { self.storage.read() }

    /// Get a [`RwLockWriteGuard`] to the [`BlockStorage`].
    ///
    /// ---
    ///
    /// [`Note`](RwLock::write): This may cause a deadlock if the lock is not
    /// released.
    pub fn write(&mut self) -> RwLockWriteGuard<'_, BlockStorage<V>> { self.storage.write() }
}

impl<V: Version> Default for BlockRegistry<V>
where
    VanillaResolver: BlockStateResolver<V>,
{
    fn default() -> Self { Self { storage: Arc::new(RwLock::new(BlockStorage::new())) } }
}
