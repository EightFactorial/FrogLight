//! TODO

use alloc::vec::Vec;
#[cfg(feature = "std")]
use core::any::TypeId;

#[cfg(feature = "std")]
use arc_swap::ArcSwap;
use froglight_common::prelude::Identifier;

use crate::entity::{EntityBundle, EntityMetadata, GlobalId};

/// A thread-safe container for a [`EntityStorage`].
#[cfg(feature = "std")]
pub struct GlobalEntityStorage {
    storage: ArcSwap<EntityStorage>,
    version_ty: TypeId,
}

#[cfg(feature = "std")]
impl GlobalEntityStorage {
    /// Create a new [`GlobalEntityStorage`] with the given [`EntityStorage`].
    #[must_use]
    pub fn new<T: 'static>(storage: EntityStorage) -> Self {
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
impl core::ops::Deref for GlobalEntityStorage {
    type Target = ArcSwap<EntityStorage>;

    fn deref(&self) -> &Self::Target { &self.storage }
}
#[cfg(feature = "std")]
impl core::ops::DerefMut for GlobalEntityStorage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.storage }
}

// -------------------------------------------------------------------------------------------------

/// A container for block data storage.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EntityStorage {
    inner: StorageInner,
}

/// The internal representation of a [`EntityStorage`].
#[derive(Debug, Clone)]
enum StorageInner {
    /// Dynamic storage allocated at runtime.
    Runtime(Vec<&'static EntityMetadata>),
    /// Static storage allocated at compile time.
    Static(&'static [&'static EntityMetadata]),
}

impl EntityStorage {
    /// Create a new static [`EntityStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided slice is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    pub const unsafe fn new_static(slice: &'static [&'static EntityMetadata]) -> Self {
        Self { inner: StorageInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`EntityStorage`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided vec is valid, with one entry
    /// per [`GlobalId`] in ascending order.
    #[must_use]
    pub const unsafe fn new_runtime(vec: Vec<&'static EntityMetadata>) -> Self {
        Self { inner: StorageInner::Runtime(vec) }
    }

    /// Get the [`Entity`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_entity(&self, id: GlobalId) -> Option<EntityBundle> {
        self.get_metadata(id).map(EntityBundle::new_from)
    }

    /// Get the [`Entity`] for a given [`Identifier`].
    #[must_use]
    pub fn get_entity_by_identifier(&self, identifier: &Identifier<'_>) -> Option<EntityBundle> {
        self.to_ref()
            .iter()
            .find(|&&meta| meta.identifier() == identifier)
            .map(|&meta| EntityBundle::new_from(meta))
    }

    /// Get the [`EntityMetadata`] for a given [`GlobalId`].
    #[must_use]
    pub fn get_metadata(&self, id: GlobalId) -> Option<&'static EntityMetadata> {
        self.to_ref().get(id.into_inner() as usize).copied()
    }

    /// Get an immutable reference to underlying storage.
    #[must_use]
    pub const fn to_ref(&self) -> &[&'static EntityMetadata] {
        match self.inner {
            StorageInner::Runtime(ref vec) => vec.as_slice(),
            StorageInner::Static(slice) => slice,
        }
    }

    /// Get a mutable reference to underlying storage.
    ///
    /// If the storage is static, it will be converted into a dynamic storage.
    #[must_use]
    pub fn to_mut(&mut self) -> &mut Vec<&'static EntityMetadata> {
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
/// [`EntityVersion`](crate::version::EntityVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(feature = "std")]
macro_rules! implement_entities {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@global $version => $($tt)*);
    };
}

/// A macro helper for implementing
/// [`EntityVersion`](crate::version::EntityVersion) for a given
/// [`Version`](froglight_common::version::Version).
///
/// This macro has will determine whether to generate a global storage constant
/// based on enabled features.
#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! implement_entities {
    ($version:ty => $($tt:tt)*) => {
        $crate::__implement_storage_inner!(@local {}, $version => $($tt)*);
    };
}

/// A hidden internal macro for the [`implement_blocks`] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_storage_inner {
    (@global $version:ty => unsafe { $($tt:tt)* }, read: $read:block, write: $write:block ) => {
        $crate::__implement_storage_inner!(
            @local {
                const ENTITY: &'static std::sync::LazyLock<$crate::storage::GlobalEntityStorage> = {
                    static STATIC: std::sync::LazyLock<$crate::storage::GlobalEntityStorage> = std::sync::LazyLock::new(|| {
                        $crate::storage::GlobalEntityStorage::new::<$version>(<$version as $crate::version::EntityVersion>::new_entity())
                    });
                    &STATIC
                };
            },
            $version => unsafe { $($tt)* },
            read: $read,
            write: $write
        );
    };
    (@local {$($constant:tt)*}, $version:ty => unsafe { $($tt:tt)* }, read: $read:block, write: $write:block )  => {
        impl $crate::version::EntityVersion for $version {
            $($constant)*

            fn new_entity() -> $crate::storage::EntityStorage {
               unsafe { $($tt)* }
            }

            #[cfg(feature = "facet")]
            const DATATYPE_DESERIALIZE: fn(
                &mut facet_minecraft::deserialize::InputCursor,
            ) -> Result<$crate::generated::datatype::EntityDataType, facet_minecraft::deserialize::error::DeserializeValueError> = $read;

            #[cfg(feature = "facet")]
            const DATATYPE_SERIALIZE: for<'input, 'facet> fn(
                &'facet (),
                &'input $crate::generated::datatype::EntityDataType,
                &mut dyn facet_minecraft::serialize::buffer::SerializeWriter,
            ) -> Result<(), facet_minecraft::serialize::error::SerializeIterError<'input, 'facet>> = $write;
        }
    };
}
