//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(all(not(feature = "async"), feature = "std", not(feature = "parking_lot")))]
pub(super) use std::sync::RwLock;

#[cfg(feature = "async")]
pub(super) use async_lock::RwLock;
#[cfg(all(not(feature = "async"), feature = "parking_lot"))]
pub(super) use parking_lot::RwLock;

#[cfg(feature = "alloc")]
use crate::block::BlockMetadata;
use crate::block::{Block, GlobalId, StateId};

/// A thread-safe container for a [`BlockStorage`].
#[repr(transparent)]
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
pub struct GlobalBlockStorage {
    storage: RwLock<BlockStorage>,
}

impl GlobalBlockStorage {
    /// Create a new [`GlobalBlockStorage`].
    #[must_use]
    pub const fn new(storage: BlockStorage) -> Self { Self { storage: RwLock::new(storage) } }

    /// Get a reference to the underlying [`RwLock`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> &RwLock<BlockStorage> { &self.storage }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn read(&self) -> async_lock::RwLockReadGuard<'_, BlockStorage> {
        self.storage.read_blocking()
    }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, BlockStorage> { self.storage.read() }

    /// Acquire a read lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, BlockStorage> {
        self.storage.read().expect("RwLock was poisoned!")
    }

    /// Acquire a read lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn read_async(&self) -> async_lock::RwLockReadGuard<'_, BlockStorage> {
        self.storage.read().await
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn write(&self) -> async_lock::RwLockWriteGuard<'_, BlockStorage> {
        self.storage.write_blocking()
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, BlockStorage> { self.storage.write() }

    /// Acquire a write lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, BlockStorage> {
        self.storage.write().expect("RwLock was poisoned!")
    }

    /// Acquire a write lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn write_async(&self) -> async_lock::RwLockWriteGuard<'_, BlockStorage> {
        self.storage.write().await
    }
}

// -------------------------------------------------------------------------------------------------

/// A container for block data storage.
#[repr(transparent)]
pub struct BlockStorage {
    inner: StorageInner,
}

/// The internal representation of a [`BlockStorage`].
enum StorageInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(Vec<&'static BlockMetadata>),
    /// Static storage allocated at compile time.
    Static(&'static [&'static BlockMetadata]),
}

impl BlockStorage {
    /// Create a new static [`BlockStorage`].
    #[must_use]
    pub const fn new_static(slice: &'static [&'static BlockMetadata]) -> Self {
        Self { inner: StorageInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`BlockStorage`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn new_runtime(vec: Vec<&'static BlockMetadata>) -> Self {
        Self { inner: StorageInner::Runtime(vec) }
    }

    /// Get the [`Block`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_block(&self, id: GlobalId) -> Option<Block> {
        let meta = self.get_metadata(id)?;
        let state = id.into_inner().saturating_sub(meta.base_id().into_inner());
        let state = StateId::new(u16::try_from(state).ok()?);
        if state <= meta.state_max() {
            // SAFETY: We just checked if the state is valid for this metadata.
            Some(unsafe { Block::new_unchecked(state, meta) })
        } else {
            None
        }
    }

    /// Get the [`BlockMetadata`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_metadata(&self, id: GlobalId) -> Option<&'static BlockMetadata> {
        self.to_ref().get(id.into_inner() as usize).copied()
    }

    /// Acquires an immutable reference to underlying storage.
    #[must_use]
    pub const fn to_ref(&self) -> &[&'static BlockMetadata] {
        match self.inner {
            #[cfg(feature = "alloc")]
            StorageInner::Runtime(ref vec) => vec.as_slice(),
            StorageInner::Static(slice) => slice,
        }
    }

    /// Acquires a mutable reference to underlying storage.
    ///
    /// If the storage is static, it will be converted into a dynamic storage.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_mut(&mut self) -> &mut Vec<&'static BlockMetadata> {
        match self.inner {
            StorageInner::Runtime(ref mut vec) => vec,
            StorageInner::Static(slice) => {
                *self = Self { inner: StorageInner::Runtime(Vec::from(slice)) };
                match self.inner {
                    StorageInner::Runtime(ref mut vec) => vec,
                    StorageInner::Static(_) => unreachable!(),
                }
            }
        }
    }
}
