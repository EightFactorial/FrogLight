//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use core::any::TypeId;
#[cfg(feature = "std")]
pub use std::sync::LazyLock;

#[cfg(feature = "std")]
use arc_swap::ArcSwap;
#[cfg(all(feature = "once_cell", not(feature = "std")))]
pub use once_cell::sync::OnceCell as LazyLock;

use crate::biome::{Biome, BiomeMetadata, GlobalId};

/// A thread-safe container for a [`BiomeStorage`].
#[cfg(feature = "std")]
pub struct GlobalBiomeStorage {
    storage: ArcSwap<BiomeStorage>,
    version_ty: TypeId,
}

#[cfg(feature = "std")]
impl GlobalBiomeStorage {
    /// Create a new [`GlobalBiomeStorage`] with the given [`BiomeStorage`].
    #[must_use]
    pub fn new<T: 'static>(storage: BiomeStorage) -> Self {
        Self {
            storage: ArcSwap::new(alloc::sync::Arc::new(storage)),
            version_ty: TypeId::of::<T>(),
        }
    }

    /// Get the [`TypeId`] of the
    /// [`Version`](froglight_common::version::Version) this storage belongs to.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version_ty }
}

#[cfg(feature = "std")]
impl core::ops::Deref for GlobalBiomeStorage {
    type Target = ArcSwap<BiomeStorage>;

    fn deref(&self) -> &Self::Target { &self.storage }
}
#[cfg(feature = "std")]
impl core::ops::DerefMut for GlobalBiomeStorage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.storage }
}

// -------------------------------------------------------------------------------------------------

/// A container for Biome data storage.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct BiomeStorage {
    inner: StorageInner,
}

/// The internal representation of a [`BiomeStorage`].
#[derive(Debug, Clone)]
enum StorageInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(Vec<&'static BiomeMetadata>),
    /// Static storage allocated at compile time.
    Static(&'static [&'static BiomeMetadata]),
}

impl BiomeStorage {
    /// Create a new static [`BiomeStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided slice is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    pub const unsafe fn new_static(slice: &'static [&'static BiomeMetadata]) -> Self {
        Self { inner: StorageInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`BiomeStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided vec is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const unsafe fn new_runtime(vec: Vec<&'static BiomeMetadata>) -> Self {
        Self { inner: StorageInner::Runtime(vec) }
    }

    /// Get the [`Biome`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_biome(&self, id: GlobalId) -> Option<Biome> {
        self.get_metadata(id).map(Biome::new_from)
    }

    /// Get the [`BiomeMetadata`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_metadata(&self, id: GlobalId) -> Option<&'static BiomeMetadata> {
        self.to_ref().get(id.into_inner() as usize).copied()
    }

    /// Get an immutable reference to underlying storage.
    #[must_use]
    pub const fn to_ref(&self) -> &[&'static BiomeMetadata] {
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
    pub fn to_mut(&mut self) -> &mut Vec<&'static BiomeMetadata> {
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
/// [`BiomeVersion`](crate::version::BiomeVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(feature = "std")]
macro_rules! implement_biomes {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@global $version => $($tt)*);
    };
}

/// A macro helper for implementing
/// [`BiomeVersion`](crate::version::BiomeVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! implement_biomes {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@local {}, $version => $($tt)*);
    };
}

/// A hidden internal macro for the [`implement_biomes`] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_storage_inner {
    (@global $version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(
            @local {
                const BIOMES: &'static $crate::storage::LazyLock<$crate::storage::GlobalBiomeStorage> = {
                    static STATIC: $crate::storage::LazyLock<$crate::storage::GlobalBiomeStorage> = $crate::storage::LazyLock::new(|| {
                        $crate::storage::GlobalBiomeStorage::new::<$version>(<$version as $crate::version::BiomeVersion>::new_biomes())
                    });
                    &STATIC
                };
            },
            $version => $($tt)*
        );
    };
    (@local {$($constant:tt)*}, $version:ty => $($tt:tt)*) => {
        impl $crate::version::BiomeVersion for $version {
            $($constant)*

            fn new_biomes() -> $crate::storage::BiomeStorage {
                $($tt)*
            }
        }
    };
}
