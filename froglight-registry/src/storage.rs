//! TODO
#![allow(
    clippy::match_wildcard_for_single_variants,
    reason = "Enums have additional variants enabled with the `alloc` feature"
)]

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(all(not(feature = "async"), feature = "std", not(feature = "parking_lot")))]
use std::sync::RwLock;

#[cfg(feature = "async")]
use async_lock::RwLock;
#[cfg(feature = "alloc")]
use foldhash::fast::RandomState;
use froglight_common::identifier::Identifier;
#[doc(hidden)]
pub use froglight_common::identifier::Identifier as __Identifier;
#[cfg(feature = "alloc")]
use indexmap::IndexMap;
#[cfg(all(not(feature = "async"), feature = "parking_lot"))]
use parking_lot::RwLock;

/// A thread-safe container for a [`RegistrySetStorage`].
#[repr(transparent)]
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
pub struct GlobalRegistrySetStorage {
    storage: RwLock<RegistrySetStorage>,
}

#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
impl GlobalRegistrySetStorage {
    /// Create a new [`GlobalRegistrySetStorage`].
    #[must_use]
    pub const fn new(storage: RegistrySetStorage) -> Self { Self { storage: RwLock::new(storage) } }

    /// Get a reference to the underlying [`RwLock`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> &RwLock<RegistrySetStorage> { &self.storage }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn read(&self) -> async_lock::RwLockReadGuard<'_, RegistrySetStorage> {
        self.storage.read_blocking()
    }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, RegistrySetStorage> {
        self.storage.read()
    }

    /// Acquire a read lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, RegistrySetStorage> {
        self.storage.read().expect("RwLock was poisoned!")
    }

    /// Acquire a read lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn read_async(&self) -> async_lock::RwLockReadGuard<'_, RegistrySetStorage> {
        self.storage.read().await
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn write(&self) -> async_lock::RwLockWriteGuard<'_, RegistrySetStorage> {
        self.storage.write_blocking()
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, RegistrySetStorage> {
        self.storage.write()
    }

    /// Acquire a write lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, RegistrySetStorage> {
        self.storage.write().expect("RwLock was poisoned!")
    }

    /// Acquire a write lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn write_async(&self) -> async_lock::RwLockWriteGuard<'_, RegistrySetStorage> {
        self.storage.write().await
    }
}

// -------------------------------------------------------------------------------------------------

/// A container for registry set storage.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RegistrySetStorage {
    inner: SetStorageInner,
}

/// The internal representation of a [`RegistrySetStorage`].
#[derive(Debug, Clone)]
enum SetStorageInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(IndexMap<Identifier<'static>, RegistryStorage, RandomState>),
    /// Static storage allocated at compile time.
    Static(&'static [(Identifier<'static>, RegistryStorage)]),
}

impl RegistrySetStorage {
    /// Create a new static [`RegistrySetStorage`].
    #[must_use]
    pub const fn new_static(slice: &'static [(Identifier<'static>, RegistryStorage)]) -> Self {
        Self { inner: SetStorageInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`RegistrySetStorage`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn new_runtime(
        map: IndexMap<Identifier<'static>, RegistryStorage, RandomState>,
    ) -> Self {
        Self { inner: SetStorageInner::Runtime(map) }
    }

    /// Get the number of registries stored.
    #[must_use]
    pub fn len(&self) -> usize {
        match &self.inner {
            #[cfg(feature = "alloc")]
            SetStorageInner::Runtime(map) => map.len(),
            SetStorageInner::Static(slice) => slice.len(),
        }
    }

    /// Return `true` if there are no registries stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match &self.inner {
            #[cfg(feature = "alloc")]
            SetStorageInner::Runtime(map) => map.is_empty(),
            SetStorageInner::Static(slice) => slice.is_empty(),
        }
    }

    /// Get a reference to a [`RegistryStorage`] by its [`Identifier`].
    #[must_use]
    pub fn get<T: AsRef<str> + ?Sized>(&self, identifier: &T) -> Option<&RegistryStorage> {
        let identifier = identifier.as_ref();
        match &self.inner {
            #[cfg(feature = "alloc")]
            SetStorageInner::Runtime(map) => map.get(identifier),
            SetStorageInner::Static(slice) => {
                slice.iter().find(|(id, _)| id.as_str() == identifier).map(|(_, storage)| storage)
            }
        }
    }

    /// Get a mutable reference to a [`RegistryStorage`] by its [`Identifier`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn get_mut<T: AsRef<str> + ?Sized>(
        &mut self,
        identifier: &T,
    ) -> Option<&mut RegistryStorage> {
        self.to_mut().get_mut(identifier.as_ref())
    }

    /// Insert a new [`RegistryStorage`] with the given [`Identifier`].
    #[cfg(feature = "alloc")]
    pub fn insert(&mut self, identifier: Identifier<'static>, storage: RegistryStorage) {
        self.to_mut().insert(identifier, storage);
    }

    /// Remove a [`RegistryStorage`] by its [`Identifier`].
    ///
    /// Returns the removed [`RegistryStorage`] if it existed.
    ///
    /// ## Note
    ///
    /// See [`IndexMap::swap_remove`] for more details.
    #[cfg(feature = "alloc")]
    pub fn swap_remove(&mut self, identifier: &str) -> Option<RegistryStorage> {
        self.to_mut().swap_remove(identifier)
    }

    /// Remove a [`RegistryStorage`] by its [`Identifier`].
    ///
    /// Returns the removed [`RegistryStorage`] if it existed.
    ///
    /// ## Note
    ///
    /// See [`IndexMap::shift_remove`] for more details.
    #[cfg(feature = "alloc")]
    pub fn shift_remove(&mut self, identifier: &str) -> Option<RegistryStorage> {
        self.to_mut().shift_remove(identifier)
    }

    /// Get a mutable reference to underlying storage.
    ///
    /// If the storage is static, it will be converted into a dynamic storage.
    #[cfg(feature = "alloc")]
    pub fn to_mut(&mut self) -> &mut IndexMap<Identifier<'static>, RegistryStorage, RandomState> {
        match self.inner {
            SetStorageInner::Runtime(ref mut map) => map,
            SetStorageInner::Static(slice) => {
                let map = slice.iter().cloned().collect();
                self.inner = SetStorageInner::Runtime(map);
                match self.inner {
                    SetStorageInner::Runtime(ref mut map) => map,
                    _ => unreachable!(),
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A container for registry storage.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RegistryStorage {
    inner: StorageInner,
}

/// The internal representation of a [`RegistryStorage`].
#[derive(Debug, Clone)]
enum StorageInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(Vec<Identifier<'static>>),
    /// Static storage allocated at compile time.
    Static(&'static [Identifier<'static>]),
}

impl RegistryStorage {
    /// Create a new static [`RegistryStorage`].
    #[must_use]
    pub const fn new_static(slice: &'static [Identifier<'static>]) -> Self {
        Self { inner: StorageInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`RegistryStorage`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn new_runtime(vec: Vec<Identifier<'static>>) -> Self {
        Self { inner: StorageInner::Runtime(vec) }
    }

    /// Get the number of entries stored.
    #[must_use]
    pub const fn len(&self) -> usize {
        match &self.inner {
            #[cfg(feature = "alloc")]
            StorageInner::Runtime(vec) => vec.len(),
            StorageInner::Static(slice) => slice.len(),
        }
    }

    /// Return `true` if there are no entries stored.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match &self.inner {
            #[cfg(feature = "alloc")]
            StorageInner::Runtime(vec) => vec.is_empty(),
            StorageInner::Static(slice) => slice.is_empty(),
        }
    }

    /// Get an [`Identifier`] by its registry index.
    #[must_use]
    pub const fn get_name(&self, index: usize) -> Option<&Identifier<'static>> {
        if index < self.len() { Some(&self.as_slice()[index]) } else { None }
    }

    /// Get the index of an [`Identifier`] by its name.
    #[must_use]
    pub fn get_index<T: AsRef<str> + ?Sized>(&self, name: &T) -> Option<usize> {
        let name = name.as_ref();
        self.as_slice().iter().position(|id| id.as_str() == name)
    }

    /// Get the index of an [`Identifier`] by its name.
    #[must_use]
    pub fn get_index_const(&self, name: &str) -> Option<usize> {
        let slice = self.as_slice();
        let mut index = 0;
        while index < slice.len() {
            if slice[index].as_str() == name {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    /// Get a reference to the underlying slice of identifiers.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[Identifier<'static>] { self.to_ref() }

    /// Get a reference to the underlying slice of identifiers.
    #[must_use]
    pub const fn to_ref(&self) -> &[Identifier<'static>] {
        match &self.inner {
            #[cfg(feature = "alloc")]
            StorageInner::Runtime(vec) => vec.as_slice(),
            StorageInner::Static(slice) => slice,
        }
    }

    /// Get a mutable reference to underlying storage.
    ///
    /// If the storage is static, it will be converted into a dynamic storage.
    #[cfg(feature = "alloc")]
    pub fn to_mut(&mut self) -> &mut Vec<Identifier<'static>> {
        match self.inner {
            StorageInner::Runtime(ref mut vec) => vec,
            StorageInner::Static(slice) => {
                let vec = slice.to_vec();
                self.inner = StorageInner::Runtime(vec);
                match self.inner {
                    StorageInner::Runtime(ref mut vec) => vec,
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl AsRef<[Identifier<'static>]> for RegistryStorage {
    #[inline]
    fn as_ref(&self) -> &[Identifier<'static>] { self.as_slice() }
}

// -------------------------------------------------------------------------------------------------

/// A macro helper for implementing
/// [`RegistryVersion`](crate::version::RegistryVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
macro_rules! implement_registry {
    ($version:ty => { $($tt:tt)* })  => {
        $crate::__implement_storage_inner!(@global $version => { $($tt)* });
    };
}

/// A macro helper for implementing [`RegistryVersion`] for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
macro_rules! implement_registry {
    ($version:ty => { $($tt:tt)* }) => {
        $crate::__implement_storage_inner!(@local {}, $version => { $($tt)* });
    };
}

/// A hidden internal macro for the [`implement_registry`] macro.
///
/// Parses the following syntax:
/// ```rust,ignore
/// implement_registry! {
///     TestVersion => {
///         "test:example_a" => [
///             "test:example_a_a",
///             "test:example_a_b",
///             "test:example_a_c",
///         ],
///         "test:example_b" => [
///             "test:example_b_a",
///             "test:example_b_b",
///         ],
///         // ...
///    }
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_storage_inner {
    (@global $version:ty => { $($tt:tt)* } ) => {
        $crate::__implement_storage_inner!(
            @local {
                const REGISTRIES: &'static $crate::storage::GlobalRegistrySetStorage = {
                    static STATIC: $crate::storage::GlobalRegistrySetStorage = $crate::storage::GlobalRegistrySetStorage::new(
                        $crate::storage::RegistrySetStorage::new_static(ENTRIES)
                    );
                    &STATIC
                };
            },
            $version => { $($tt)* }
        );
    };
    (@local {$($constant:tt)*}, $version:ty => { $($tt:tt)* } ) => {
        #[doc(hidden)]
        mod __registry_storage_impl {
            #[allow(unused_imports, reason = "Macro generated code")]
            use super::*;

            static ENTRIES: &'static [( $crate::storage::__Identifier<'static>, $crate::storage::RegistryStorage )] =
                $crate::__implement_storage_inner!(@parse { $($tt)* });

            impl $crate::version::RegistryVersion for $version {
                $($constant)*

                fn new_registries() -> $crate::storage::RegistrySetStorage {
                    $crate::storage::RegistrySetStorage::new_static(ENTRIES)
                }
            }
        }
    };

    (@parse { $( $id:expr => [ $( $entry:expr ),* $(,)? ] ),* $(,)? }) => {{
        &[
            $(
                (
                    $crate::storage::__Identifier::new_static($id),
                    $crate::storage::RegistryStorage::new_static({
                        static INNER: &'static [ $crate::storage::__Identifier<'static> ] =
                        &[
                            $(
                                $crate::storage::__Identifier::new_static($entry),
                            )*
                        ];
                        INNER
                    }),
                ),
            )*
        ]
    }}
}
