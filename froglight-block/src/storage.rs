//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use core::any::TypeId;

#[cfg(feature = "std")]
use arc_swap::ArcSwap;

use crate::block::{Block, BlockMetadata, GlobalId, StateId};

/// A thread-safe container for a [`BlockStorage`].
#[cfg(feature = "std")]
pub struct GlobalBlockStorage {
    storage: ArcSwap<BlockStorage>,
    version_ty: TypeId,
}

#[cfg(feature = "std")]
impl GlobalBlockStorage {
    /// Create a new [`GlobalBlockStorage`] with the given [`BlockStorage`].
    #[must_use]
    pub fn new<T: 'static>(storage: BlockStorage) -> Self {
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
impl core::ops::Deref for GlobalBlockStorage {
    type Target = ArcSwap<BlockStorage>;

    fn deref(&self) -> &Self::Target { &self.storage }
}
#[cfg(feature = "std")]
impl core::ops::DerefMut for GlobalBlockStorage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.storage }
}

// -------------------------------------------------------------------------------------------------

/// A container for block data storage.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct BlockStorage {
    inner: StorageInner,
}

/// The internal representation of a [`BlockStorage`].
#[derive(Debug, Clone)]
enum StorageInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(Vec<&'static BlockMetadata>),
    /// Static storage allocated at compile time.
    Static(&'static [&'static BlockMetadata]),
}

impl BlockStorage {
    /// Create a new static [`BlockStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided slice is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    pub const unsafe fn new_static(slice: &'static [&'static BlockMetadata]) -> Self {
        Self { inner: StorageInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`BlockStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided vec is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const unsafe fn new_runtime(vec: Vec<&'static BlockMetadata>) -> Self {
        Self { inner: StorageInner::Runtime(vec) }
    }

    /// Get the [`Block`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_block(&self, id: GlobalId) -> Option<Block> {
        let metadata = self.get_metadata(id)?;
        let state = id.into_inner().saturating_sub(metadata.base_id().into_inner());
        let state = StateId::new(u16::try_from(state).ok()?);

        if state.into_inner() < metadata.state_count() {
            // SAFETY: We just checked if the state is valid for this metadata.
            Some(unsafe { Block::new_unchecked(state, metadata) })
        } else {
            None
        }
    }

    /// Get the [`BlockMetadata`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_metadata(&self, id: GlobalId) -> Option<&'static BlockMetadata> {
        self.to_ref().get(id.into_inner() as usize).copied()
    }

    /// Get an immutable reference to underlying storage.
    #[must_use]
    pub const fn to_ref(&self) -> &[&'static BlockMetadata] {
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

// -------------------------------------------------------------------------------------------------

/// A macro helper for implementing
/// [`BlockVersion`](crate::version::BlockVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(feature = "std")]
macro_rules! implement_blocks {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@global $version => $($tt)*);
    };
}

/// A macro helper for implementing
/// [`BlockVersion`](crate::version::BlockVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! implement_blocks {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@local {}, $version => $($tt)*);
    };
}

/// A hidden internal macro for the [`implement_blocks`] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_storage_inner {
    (@global $version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(
            @local {
                const BLOCKS: &'static std::sync::LazyLock<$crate::storage::GlobalBlockStorage> = {
                    static STATIC: std::sync::LazyLock<$crate::storage::GlobalBlockStorage> = std::sync::LazyLock::new(|| {
                        $crate::storage::GlobalBlockStorage::new::<$version>(<$version as $crate::version::BlockVersion>::new_blocks())
                    });
                    &STATIC
                };
            },
            $version => $($tt)*
        );
    };
    (@local {$($constant:tt)*}, $version:ty => $($tt:tt)*) => {
        impl $crate::version::BlockVersion for $version {
            $($constant)*

            fn new_blocks() -> $crate::storage::BlockStorage {
                $($tt)*
            }
        }
    };
}
