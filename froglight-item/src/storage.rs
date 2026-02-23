//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use arc_swap::ArcSwap;

use crate::item::{GlobalId, Item, ItemMetadata};

/// A thread-safe container for a [`ItemStorage`].
#[repr(transparent)]
#[cfg(feature = "std")]
pub struct GlobalItemStorage {
    storage: ArcSwap<ItemStorage>,
}

#[cfg(feature = "std")]
impl GlobalItemStorage {
    /// Create a new [`GlobalItemStorage`] from a given [`ItemStorage`].
    #[must_use]
    pub fn new(storage: ItemStorage) -> Self {
        Self { storage: ArcSwap::new(alloc::sync::Arc::new(storage)) }
    }
}

#[cfg(feature = "std")]
impl core::ops::Deref for GlobalItemStorage {
    type Target = ArcSwap<ItemStorage>;

    fn deref(&self) -> &Self::Target { &self.storage }
}
#[cfg(feature = "std")]
impl core::ops::DerefMut for GlobalItemStorage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.storage }
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

    /// Get the [`Item`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_item(&self, id: GlobalId) -> Option<Item> {
        self.get_metadata(id).map(Item::new_from)
    }

    /// Get the [`ItemMetadata`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_metadata(&self, id: GlobalId) -> Option<&'static ItemMetadata> {
        self.to_ref().get(id.into_inner() as usize).copied()
    }

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
#[cfg(feature = "std")]
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
#[cfg(not(feature = "std"))]
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
                const ITEMS: &'static std::sync::LazyLock<$crate::storage::GlobalItemStorage> = {
                    static STATIC: std::sync::LazyLock<$crate::storage::GlobalItemStorage> = std::sync::LazyLock::new(|| {
                        $crate::storage::GlobalItemStorage::new(<$version as $crate::version::ItemVersion>::new_items())
                    });
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
