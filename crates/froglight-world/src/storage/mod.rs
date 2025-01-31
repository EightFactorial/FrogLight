//! TODO

use std::any::TypeId;

use bevy_ecs::{component::Component, reflect::ReflectComponent, system::Resource};
use bevy_reflect::Reflect;
use bevy_utils::TypeIdMap;
use froglight_common::Identifier;
use glam::IVec3;

mod stored;
pub use stored::StoredChunk;

mod types;
pub use types::{End, Nether, Overworld, Storable};

use crate::chunk::VecChunk;

/// A storage for chunks.
#[derive(Default, Resource)]
pub struct ChunkStorage(TypeIdMap<Box<dyn Storable>>);

impl ChunkStorage {
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

impl ChunkStorage {
    /// Get the storage identifier of a chunk in the [`ChunkStorage`].
    pub fn identifier(&self, handle: &ChunkHandle) -> Option<&'static Identifier> {
        self.get_untyped(handle.type_id).map(Storable::identifier)
    }

    /// Insert a chunk into the [`ChunkStorage`].
    pub fn insert_chunk<T: Storable>(&mut self, chunk: VecChunk) -> Option<ChunkHandle> {
        self.get_untyped_mut(TypeId::of::<T>()).map(|storage| storage.insert_chunk(chunk))
    }
    /// Remove a chunk from the [`ChunkStorage`].
    pub fn remove_chunk(&mut self, handle: ChunkHandle) {
        if let Some(storage) = self.get_untyped_mut(handle.type_id) {
            storage.remove_chunk(handle);
        }
    }

    /// Get a block from the [`ChunkStorage`].
    #[must_use]
    pub fn get_block(&self, handle: &ChunkHandle, position: IVec3) -> Option<u32> {
        self.get_untyped(handle.type_id)?.get_block(handle, position)
    }
    /// Set a block in the [`ChunkStorage`].
    #[must_use]
    pub fn set_block(&mut self, handle: &ChunkHandle, position: IVec3, block: u32) -> Option<u32> {
        self.get_untyped_mut(handle.type_id)?.set_block(handle, position, block)
    }
}

/// A handle to a chunk in the [`ChunkStorage`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Debug, PartialEq, Hash, Component)]
pub struct ChunkHandle {
    type_id: TypeId,
    index: u32,
}
