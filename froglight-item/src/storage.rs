//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(all(not(feature = "async"), feature = "std", not(feature = "parking_lot")))]
use std::sync::RwLock;

#[cfg(feature = "async")]
use async_lock::RwLock;
#[cfg(all(not(feature = "async"), feature = "parking_lot"))]
use parking_lot::RwLock;

// use crate::item::{Item, ItemMetadata, GlobalId};

type ItemMetadata = ();

/// A thread-safe container for a [`ItemStorage`].
#[repr(transparent)]
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
pub struct GlobalItemStorage {
    storage: RwLock<ItemStorage>,
}

#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
impl GlobalItemStorage {
    /// Create a new [`GlobalItemStorage`].
    #[must_use]
    pub const fn new(storage: ItemStorage) -> Self { Self { storage: RwLock::new(storage) } }

    /// Get a reference to the underlying [`RwLock`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> &RwLock<ItemStorage> { &self.storage }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn read(&self) -> async_lock::RwLockReadGuard<'_, ItemStorage> {
        self.storage.read_blocking()
    }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, ItemStorage> { self.storage.read() }

    /// Acquire a read lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, ItemStorage> {
        self.storage.read().expect("RwLock was poisoned!")
    }

    /// Acquire a read lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn read_async(&self) -> async_lock::RwLockReadGuard<'_, ItemStorage> {
        self.storage.read().await
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn write(&self) -> async_lock::RwLockWriteGuard<'_, ItemStorage> {
        self.storage.write_blocking()
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, ItemStorage> { self.storage.write() }

    /// Acquire a write lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, ItemStorage> {
        self.storage.write().expect("RwLock was poisoned!")
    }

    /// Acquire a write lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn write_async(&self) -> async_lock::RwLockWriteGuard<'_, ItemStorage> {
        self.storage.write().await
    }
}

// -------------------------------------------------------------------------------------------------

/// A container for item data storage.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ItemStorage {
    inner: StorageInner,
}

/// The internal representation of a [`ItemStorage`].
#[derive(Debug, Clone)]
enum StorageInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(Vec<&'static ItemMetadata>),
    /// Static storage allocated at compile time.
    Static(&'static [&'static ItemMetadata]),
}

impl ItemStorage {
    /// Create a new static [`ItemStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided slice is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    pub const unsafe fn new_static(slice: &'static [&'static ItemMetadata]) -> Self {
        Self { inner: StorageInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`ItemStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided vec is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const unsafe fn new_runtime(vec: Vec<&'static ItemMetadata>) -> Self {
        Self { inner: StorageInner::Runtime(vec) }
    }

    // /// Get the [`Item`] for a given [`GlobalId`].
    // #[must_use]
    // pub fn get_item(&self, id: GlobalId) -> Option<Item> {
    //     let metadata = self.get_metadata(id)?;
    //     let state =
    // id.into_inner().saturating_sub(metadata.base_id().into_inner());
    //     let state = StateId::new(u16::try_from(state).ok()?);
    //
    //     if state.into_inner() < metadata.state_count() {
    //         // SAFETY: We just checked if the state is valid for this metadata.
    //         Some(unsafe { Item::new_unchecked(state, metadata) })
    //     } else {
    //         None
    //     }
    // }

    // /// Get the [`ItemMetadata`] for a given [`GlobalId`].
    // #[must_use]
    // pub fn get_metadata(&self, id: GlobalId) -> Option<&'static ItemMetadata> {
    //     self.to_ref().get(id.into_inner() as usize).copied()
    // }

    /// Get an immutable reference to underlying storage.
    #[must_use]
    pub const fn to_ref(&self) -> &[&'static ItemMetadata] {
        match self.inner {
            #[cfg(feature = "alloc")]
            StorageInner::Runtime(ref vec) => vec.as_slice(),
            StorageInner::Static(slice) => slice,
        }
    }

    /// Get a mutable reference to underlying storage.
    ///
    /// If the storage is static, it will be converted into a dynamic storage.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_mut(&mut self) -> &mut Vec<&'static ItemMetadata> {
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

// -------------------------------------------------------------------------------------------------

/// A macro helper for implementing
/// [`ItemVersion`](crate::version::ItemVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
macro_rules! implement_items {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@global $version => $($tt)*);
    };
}

/// A macro helper for implementing
/// [`ItemVersion`](crate::version::ItemVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
macro_rules! implement_items {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@local {}, $version => $($tt)*);
    };
}

/// A hidden internal macro for the [`implement_items`] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_storage_inner {
    (@global $version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(
            @local {
                const ITEMS: &'static $crate::storage::GlobalItemStorage = {
                    static STATIC: $crate::storage::GlobalItemStorage = $crate::storage::GlobalItemStorage::new(
                        $($tt)*
                    );
                    &STATIC
                };
            },
            $version => $($tt)*
        );
    };
    (@local {$($constant:tt)*}, $version:ty => $($tt:tt)*) => {
        impl $crate::version::ItemVersion for $version {
            $($constant)*

            fn new_items() -> $crate::storage::ItemStorage {
                $($tt)*
            }
        }
    };
}
