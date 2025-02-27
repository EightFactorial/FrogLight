//! TODO

use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::{component::Component, reflect::ReflectComponent, system::Resource};
use bevy_reflect::Reflect;
use bevy_utils::TypeIdMap;
use derive_more::derive::Deref;
use froglight_block::{
    block::UntypedBlock,
    resolve::BlockResolver,
    storage::{BlockStorage, GlobalBlockId},
};
use froglight_common::{identifier::Identifier, version::Version};
use glam::IVec3;

mod stored;
pub use stored::StoredChunk;

mod types;
pub use types::{End, Nether, Overworld, Storable};

use crate::chunk::VecChunk;

/// A storage for chunks.
#[derive(Default, Resource)]
pub struct ChunkStorage<V: Version>(TypeIdMap<Box<dyn Storable>>, PhantomData<V>);

impl<V: Version> ChunkStorage<V> {
    /// Insert a [`Storable`] type into the [`ChunkStorage`].
    pub fn insert<T: Storable>(&mut self, value: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(value));
    }
    /// Take a [`Storable`] type from the [`ChunkStorage`].
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn take<T: Storable>(&mut self) -> Option<T> {
        self.0
            .remove(&TypeId::of::<T>())
            .map(|value| -> T { *value.into_any().downcast().unwrap() })
    }

    /// Get an untyped [`Storable`] type from the [`ChunkStorage`].
    #[inline]
    #[must_use]
    fn get_untyped(&self, type_id: TypeId) -> Option<&dyn Storable> {
        self.0.get(&type_id).map(AsRef::as_ref)
    }
    /// Get a mutable untyped [`Storable`] type from the [`ChunkStorage`].
    #[inline]
    #[must_use]
    fn get_untyped_mut(&mut self, type_id: TypeId) -> Option<&mut dyn Storable> {
        self.0.get_mut(&type_id).map(AsMut::as_mut)
    }

    /// Get a [`Storable`] type from the [`ChunkStorage`].
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn get<T: Storable>(&self) -> Option<&T> {
        self.get_untyped(TypeId::of::<T>()).map(|value| value.as_any().downcast_ref::<T>().unwrap())
    }
    /// Get a mutable [`Storable`] type from the [`ChunkStorage`].
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn get_mut<T: Storable>(&mut self) -> Option<&mut T> {
        self.get_untyped_mut(TypeId::of::<T>())
            .map(|value| value.as_any_mut().downcast_mut::<T>().unwrap())
    }
}

impl<V: Version> ChunkStorage<V> {
    /// Get the storage identifier of a chunk in the [`ChunkStorage`].
    pub fn identifier(&self, handle: &ChunkHandle<V>) -> Option<&'static Identifier> {
        self.get_untyped(handle.0.0).map(Storable::identifier)
    }

    /// Insert a chunk into the [`ChunkStorage`].
    pub fn insert_chunk<T: Storable>(&mut self, chunk: VecChunk) -> Option<ChunkHandle<V>> {
        self.get_untyped_mut(TypeId::of::<T>())
            .map(|storage| ChunkHandle(storage.insert_chunk(chunk), PhantomData))
    }
    /// Remove a chunk from the [`ChunkStorage`].
    pub fn remove_chunk(&mut self, handle: ChunkHandle<V>) {
        if let Some(storage) = self.get_untyped_mut(handle.0.0) {
            storage.remove_chunk(handle.0);
        }
    }

    /// Get a block id from the [`ChunkStorage`].
    #[must_use]
    pub fn get_block_raw(&self, handle: &ChunkHandle<V>, position: IVec3) -> Option<u32> {
        self.get_untyped(handle.0.0)?.get_block(handle, position)
    }
    /// Set a block id in the [`ChunkStorage`].
    #[must_use]
    pub fn set_block_raw(
        &mut self,
        handle: &ChunkHandle<V>,
        position: IVec3,
        block: u32,
    ) -> Option<u32> {
        self.get_untyped_mut(handle.0.0)?.set_block(handle, position, block)
    }

    /// Get a block from the [`ChunkStorage`] with data from a [`BlockStorage`].
    ///
    /// Returns `None` if the position is out of bounds
    /// or no matching block is found.
    #[must_use]
    pub fn get_block_untyped(
        &self,
        handle: &ChunkHandle<V>,
        position: IVec3,
        storage: &BlockStorage<V>,
    ) -> Option<UntypedBlock<V>> {
        self.get_block_raw(handle, position)
            .and_then(|id| storage.get_untyped(GlobalBlockId::new_unchecked(id)))
    }
    /// Set a block in the [`ChunkStorage`] using data from a [`BlockStorage`].
    ///
    /// Returns the previous block if it was set, or
    /// `None` if the position is out of bounds or no matching block is found.
    pub fn set_block_untyped(
        &mut self,
        handle: &ChunkHandle<V>,
        position: IVec3,
        block: impl Into<UntypedBlock<V>>,
        storage: &BlockStorage<V>,
    ) -> Option<UntypedBlock<V>> {
        self.set_block_raw(handle, position, *storage.get_global(block)?)
            .and_then(|id| storage.get_untyped(GlobalBlockId::new_unchecked(id)))
    }

    /// Get a block from the [`ChunkStorage`] with data from a [`BlockStorage`].
    ///
    /// If you don't need specific block-type details,
    /// consider using [`ChunkStorage::get_block_untyped`] instead.
    ///
    /// Returns `None` if the position is out of bounds
    /// or no matching block is found.
    #[inline]
    #[must_use]
    pub fn get_block<R: BlockResolver<V>>(
        &self,
        handle: &ChunkHandle<V>,
        position: IVec3,
        storage: &BlockStorage<V>,
    ) -> Option<R::BlockEnum> {
        self.get_block_untyped(handle, position, storage).and_then(|block| R::resolve(block))
    }
    /// Set a block in the [`ChunkStorage`] using data from a [`BlockStorage`].
    ///
    /// If you don't need type details about the previous block,
    /// consider using [`ChunkStorage::set_block_untyped`] instead.
    ///
    /// Returns the previous block if it was set, or
    /// `None` if the position is out of bounds or no matching block is found.
    #[inline]
    pub fn set_block<R: BlockResolver<V>>(
        &mut self,
        handle: &ChunkHandle<V>,
        position: IVec3,
        block: impl Into<UntypedBlock<V>>,
        storage: &BlockStorage<V>,
    ) -> Option<R::BlockEnum> {
        self.set_block_untyped(handle, position, block, storage).and_then(|block| R::resolve(block))
    }
}

/// A handle to a chunk in the [`ChunkStorage`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect, Deref)]
#[reflect(Debug, PartialEq, Hash, Component)]
pub struct ChunkHandle<V: Version>(#[deref] HandleInternal, PhantomData<V>);

/// An internal handle to a chunk in the [`ChunkStorage`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Debug, PartialEq, Hash, Component)]
pub struct HandleInternal(TypeId, u32);
