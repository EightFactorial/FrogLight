//! TODO

#[cfg(all(feature = "alloc", feature = "biome_data"))]
use alloc::vec::Vec;
#[cfg(all(
    feature = "biome_data",
    not(feature = "async"),
    feature = "std",
    not(feature = "parking_lot")
))]
use std::sync::RwLock;

#[cfg(all(feature = "async", feature = "biome_data"))]
use async_lock::RwLock;
#[cfg(feature = "biome_data")]
use facet::Facet;
#[cfg(feature = "biome_data")]
use facet_format::SerializeError;
#[cfg(feature = "biome_data")]
use facet_value::ToValueError;
#[cfg(feature = "biome_data")]
use facet_value::Value;
#[cfg(feature = "biome_data")]
use facet_value::ValueError;
#[cfg(feature = "biome_data")]
use froglight_common::prelude::Identifier;
#[cfg(all(feature = "biome_data", not(feature = "async"), feature = "parking_lot"))]
use parking_lot::RwLock;

/// A set of biome attributes.
#[repr(transparent)]
#[derive(Debug)]
#[cfg(feature = "biome_data")]
pub struct BiomeAttributeSet {
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    storage: RwLock<BiomeAttributeStorage>,
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    storage: BiomeAttributeStorage,
}

#[cfg(feature = "biome_data")]
impl BiomeAttributeSet {
    /// Create an empty [`BiomeAttributeSet`] instance.
    #[must_use]
    pub const fn empty() -> Self { Self::new_static(&[]) }

    /// Create a new static [`BiomeAttributeSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided slice contains duplicate entries.
    #[must_use]
    pub const fn new_static(features: &'static [(Identifier<'static>, Value)]) -> Self {
        Self {
            #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
            storage: RwLock::new(BiomeAttributeStorage::new_static(features)),
            #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
            storage: BiomeAttributeStorage::new_static(features),
        }
    }

    /// Create a new runtime-allocated [`BiomeAttributeSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided vector contains duplicate entries.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new_runtime(vec: Vec<(Identifier<'static>, Value)>) -> Self {
        Self {
            #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
            storage: RwLock::new(BiomeAttributeStorage::new_runtime(vec)),
            #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
            storage: BiomeAttributeStorage::new_runtime(vec),
        }
    }

    /// Acquire a reference without blocking the current thread.
    #[inline]
    #[must_use]
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    pub const fn read(&self) -> &BiomeAttributeStorage { &self.storage }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn read(&self) -> async_lock::RwLockReadGuard<'_, BiomeAttributeStorage> {
        self.storage.read_blocking()
    }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, BiomeAttributeStorage> {
        self.storage.read()
    }

    /// Acquire a read lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, BiomeAttributeStorage> {
        self.storage.read().expect("RwLock was poisoned!")
    }

    /// Acquire a read lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn read_async(&self) -> async_lock::RwLockReadGuard<'_, BiomeAttributeStorage> {
        self.storage.read().await
    }

    /// Acquire a mutable reference without blocking the current thread.
    #[inline]
    #[must_use]
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    pub const fn write(&mut self) -> &mut BiomeAttributeStorage { &mut self.storage }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn write(&self) -> async_lock::RwLockWriteGuard<'_, BiomeAttributeStorage> {
        self.storage.write_blocking()
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, BiomeAttributeStorage> {
        self.storage.write()
    }

    /// Acquire a write lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, BiomeAttributeStorage> {
        self.storage.write().expect("RwLock was poisoned!")
    }

    /// Acquire a write lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn write_async(&self) -> async_lock::RwLockWriteGuard<'_, BiomeAttributeStorage> {
        self.storage.write().await
    }
}

/// A set of biome attributes.
#[derive(Debug)]
#[cfg(not(feature = "biome_data"))]
pub struct BiomeAttributeSet;

#[cfg(not(feature = "biome_data"))]
impl BiomeAttributeSet {
    /// Create an empty [`BiomeAttributeSet`] instance.
    #[must_use]
    pub const fn empty() -> Self { Self }
}

// -------------------------------------------------------------------------------------------------

/// The underlying storage for a [`BiomeAttributeSet`].
#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg(feature = "biome_data")]
pub struct BiomeAttributeStorage {
    inner: AttributeSetInner,
}

#[derive(Debug, Clone)]
#[cfg(feature = "biome_data")]
enum AttributeSetInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(Vec<(Identifier<'static>, Value)>),
    /// Static storage allocated at compile time.
    Static(&'static [(Identifier<'static>, Value)]),
}

#[cfg(feature = "biome_data")]
impl BiomeAttributeStorage {
    /// Create a new static [`BiomeAttributeSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided slice contains duplicate entries.
    #[must_use]
    pub const fn new_static(slice: &'static [(Identifier<'static>, Value)]) -> Self {
        assert_no_duplicates(slice);
        Self { inner: AttributeSetInner::Static(slice) }
    }

    /// Create a new runtime-allocated [`BiomeAttributeSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided vector contains duplicate entries.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn new_runtime(vec: Vec<(Identifier<'static>, Value)>) -> Self {
        assert_no_duplicates(vec.as_slice());
        Self { inner: AttributeSetInner::Runtime(vec) }
    }

    /// Returns `true` if the set contains the specified attribute type.
    #[must_use]
    pub fn contains<A: AttributeType>(&self) -> bool {
        self.to_ref().iter().any(|(id, _)| id == &A::IDENTIFIER)
    }

    /// Inserts the attribute type into the set,
    /// appending it to the end of the set.
    ///
    /// Returns `true` if the attribute type was inserted into the set.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute could not be converted into a
    /// [`Value`].
    #[cfg(feature = "alloc")]
    pub fn insert<A: AttributeType>(
        &mut self,
        attribute: &A,
    ) -> Result<bool, SerializeError<ToValueError>> {
        let data = attribute.to_attribute_data()?;

        // SAFETY: `self.contains` ensures no duplicates are added.
        let inserted =
            (!self.contains::<A>()).then(|| unsafe { self.to_mut().push((A::IDENTIFIER, data)) });
        Ok(inserted.is_some())
    }

    /// Removes the specified attribute type from the set.
    ///
    /// Returns `Ok(None)` if the attribute type was not present in the set.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute type could not be converted from its
    /// [`Value`].
    #[cfg(feature = "alloc")]
    pub fn remove<A: AttributeType>(&mut self) -> Result<Option<A>, ValueError> {
        if let Some(pos) = self.to_ref().iter().position(|(id, _)| id == &A::IDENTIFIER) {
            // SAFETY: Attributes are only being removed, not added.
            A::from_attribute_data(&unsafe { self.to_mut().remove(pos) }.1).map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get an immutable reference to underlying storage.
    #[must_use]
    pub const fn to_ref(&self) -> &[(Identifier<'static>, Value)] {
        match self.inner {
            #[cfg(feature = "alloc")]
            AttributeSetInner::Runtime(ref vec) => vec.as_slice(),
            AttributeSetInner::Static(slice) => slice,
        }
    }

    /// Get a mutable reference to underlying storage.
    ///
    /// If the storage is static, it will be converted into a dynamic storage.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no duplicate entries are added to the set.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub unsafe fn to_mut(&mut self) -> &mut Vec<(Identifier<'static>, Value)> {
        match self.inner {
            AttributeSetInner::Runtime(ref mut vec) => vec,
            AttributeSetInner::Static(slice) => {
                self.inner = AttributeSetInner::Runtime(Vec::from(slice));
                match self.inner {
                    AttributeSetInner::Runtime(ref mut vec) => vec,
                    AttributeSetInner::Static(_) => unreachable!(),
                }
            }
        }
    }
}

/// Asserts that the given slice contains no duplicate entries.
#[cfg(feature = "biome_data")]
const fn assert_no_duplicates(slice: &[(Identifier<'static>, Value)]) {
    let mut i = 0;
    while i < slice.len() {
        let mut j = i + 1;
        while j < slice.len() {
            assert!(
                !slice[i].0.const_eq(&slice[j].0),
                "`BiomeAttributeSet` contains duplicate entries!"
            );
            j += 1;
        }
        i += 1;
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all feature types.
#[cfg(feature = "biome_data")]
pub trait AttributeType: Facet<'static> + Sized {
    /// The [`Identifier`] of this attribute type.
    const IDENTIFIER: Identifier<'static>;

    /// Try to convert [`Value`] into this type.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    #[inline]
    fn from_attribute_data(data: &Value) -> Result<Self, ValueError> {
        facet_value::from_value::<Self>(data.clone())
    }

    /// Convert this type into [`Value`].
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    #[inline]
    fn to_attribute_data(&self) -> Result<Value, SerializeError<ToValueError>> {
        facet_value::to_value::<Self>(self)
    }
}
