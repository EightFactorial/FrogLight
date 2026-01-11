//! TODO

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(all(not(feature = "async"), feature = "std", not(feature = "parking_lot")))]
use std::sync::RwLock;

#[cfg(feature = "async")]
use async_lock::RwLock;
use froglight_common::prelude::Identifier;
#[cfg(all(not(feature = "async"), feature = "parking_lot"))]
use parking_lot::RwLock;

/// A collection of all biome feature generators.
#[derive(Debug)]
#[expect(missing_docs, reason = "TODO: Needs documentation")]
pub struct BiomeFeatures {
    pub raw_generation: BiomeFeatureSet,
    pub lakes: BiomeFeatureSet,
    pub local_modifications: BiomeFeatureSet,
    pub underground_structures: BiomeFeatureSet,
    pub surface_structures: BiomeFeatureSet,
    pub strongholds: BiomeFeatureSet,
    pub underground_ores: BiomeFeatureSet,
    pub underground_decoration: BiomeFeatureSet,
    pub fluid_springs: BiomeFeatureSet,
    pub vegetal_decoration: BiomeFeatureSet,
    pub top_layer_modification: BiomeFeatureSet,
}

impl BiomeFeatures {
    /// Create an empty [`BiomeFeatures`] instance.
    #[must_use]
    pub const fn empty() -> Self { Self::from_arrays([&[]; 11]) }

    /// Create a new [`BiomeFeatures`] from the provided set of feature arrays.
    ///
    /// # Panics
    ///
    /// Panics if any of the provided arrays contain duplicate entries.
    #[must_use]
    pub const fn from_arrays(features: [&'static [Identifier<'static>]; 11]) -> Self {
        Self {
            raw_generation: BiomeFeatureSet::new_static(features[0]),
            lakes: BiomeFeatureSet::new_static(features[1]),
            local_modifications: BiomeFeatureSet::new_static(features[2]),
            underground_structures: BiomeFeatureSet::new_static(features[3]),
            surface_structures: BiomeFeatureSet::new_static(features[4]),
            strongholds: BiomeFeatureSet::new_static(features[5]),
            underground_ores: BiomeFeatureSet::new_static(features[6]),
            underground_decoration: BiomeFeatureSet::new_static(features[7]),
            fluid_springs: BiomeFeatureSet::new_static(features[8]),
            vegetal_decoration: BiomeFeatureSet::new_static(features[9]),
            top_layer_modification: BiomeFeatureSet::new_static(features[10]),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A container for a set of biome feature types.
#[repr(transparent)]
#[derive(Debug)]
pub struct BiomeFeatureSet {
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    storage: RwLock<FeatureSetStorage>,
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    storage: FeatureSetStorage,
}

impl BiomeFeatureSet {
    /// Create a new static [`BiomeFeatureSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided slice contains duplicate entries.
    #[must_use]
    pub const fn new_static(slice: &'static [Identifier<'static>]) -> Self {
        assert_no_duplicates(slice);
        Self {
            #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
            storage: RwLock::new(FeatureSetStorage { inner: FeatureSetInner::Static(slice) }),
            #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
            storage: FeatureSetStorage { inner: FeatureSetInner::Static(slice) },
        }
    }

    /// Create a new runtime-allocated [`BiomeFeatureSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided vector contains duplicate entries.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn new_runtime(vec: Vec<Identifier<'static>>) -> Self {
        assert_no_duplicates(vec.as_slice());
        Self {
            #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
            storage: RwLock::new(FeatureSetStorage { inner: FeatureSetInner::Runtime(vec) }),
            #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
            storage: FeatureSetStorage { inner: FeatureSetInner::Runtime(vec) },
        }
    }

    /// Acquire a reference without blocking the current thread.
    #[inline]
    #[must_use]
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    pub const fn read(&self) -> &FeatureSetStorage { &self.storage }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn read(&self) -> async_lock::RwLockReadGuard<'_, FeatureSetStorage> {
        self.storage.read_blocking()
    }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, FeatureSetStorage> {
        self.storage.read()
    }

    /// Acquire a read lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, FeatureSetStorage> {
        self.storage.read().expect("RwLock was poisoned!")
    }

    /// Acquire a read lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn read_async(&self) -> async_lock::RwLockReadGuard<'_, FeatureSetStorage> {
        self.storage.read().await
    }

    /// Acquire a mutable reference without blocking the current thread.
    #[inline]
    #[must_use]
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    pub const fn write(&mut self) -> &mut FeatureSetStorage { &mut self.storage }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn write(&self) -> async_lock::RwLockWriteGuard<'_, FeatureSetStorage> {
        self.storage.write_blocking()
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, FeatureSetStorage> {
        self.storage.write()
    }

    /// Acquire a write lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, FeatureSetStorage> {
        self.storage.write().expect("RwLock was poisoned!")
    }

    /// Acquire a write lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn write_async(&self) -> async_lock::RwLockWriteGuard<'_, FeatureSetStorage> {
        self.storage.write().await
    }
}

/// Asserts that the given slice contains no duplicate entries.
const fn assert_no_duplicates(slice: &[Identifier<'static>]) {
    let mut i = 0;
    while i < slice.len() {
        let mut j = i + 1;
        while j < slice.len() {
            assert!(!slice[i].const_eq(&slice[j]), "`FeatureSet` contains duplicate entries!");
            j += 1;
        }
        i += 1;
    }
}

// -------------------------------------------------------------------------------------------------

/// The underlying storage for a [`BiomeFeatureSet`].
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct FeatureSetStorage {
    inner: FeatureSetInner,
}

#[derive(Debug, Clone)]
enum FeatureSetInner {
    /// Dynamic storage allocated at runtime.
    #[cfg(feature = "alloc")]
    Runtime(Vec<Identifier<'static>>),
    /// Static storage allocated at compile time.
    Static(&'static [Identifier<'static>]),
}

impl FeatureSetStorage {
    /// Returns `true` if the set contains the specified feature type.
    #[must_use]
    pub fn contains<F: FeatureType>(&self) -> bool { self.to_ref().contains(&F::IDENTIFIER) }

    /// Inserts the feature type into the set,
    /// appending it to the end of the set.
    ///
    /// Returns `true` if the feature type was inserted into the set.
    #[cfg(feature = "alloc")]
    pub fn insert<F: FeatureType>(&mut self) -> bool {
        // SAFETY: `self.contains` ensures no duplicates are added.
        (!self.contains::<F>()).then(|| unsafe { self.to_mut().push(F::IDENTIFIER) }).is_some()
    }

    /// Inserts the feature into the set at the given position,
    /// shifting all elements after it to the right.
    ///
    /// Returns `true` if the feature type was inserted into the set.
    #[cfg(feature = "alloc")]
    pub fn insert_at<F: FeatureType>(&mut self, index: usize) -> bool {
        // SAFETY: `self.contains` ensures no duplicates are added.
        (!self.contains::<F>())
            .then(|| unsafe { self.to_mut().insert(index, F::IDENTIFIER) })
            .is_some()
    }

    /// Inserts the feature `F1` before feature `F2` in the set.
    ///
    /// Returns `true` if the feature type was inserted into the set.
    #[cfg(feature = "alloc")]
    pub fn insert_before<F1: FeatureType, F2: FeatureType>(&mut self) -> bool {
        if let Some(pos) = self.to_ref().iter().position(|id| *id == F2::IDENTIFIER) {
            self.insert_at::<F1>(pos)
        } else {
            false
        }
    }

    /// Inserts the feature `F1` after feature `F2` in the set.
    ///
    /// Returns `true` if the feature type was inserted into the set.
    #[cfg(feature = "alloc")]
    pub fn insert_after<F1: FeatureType, F2: FeatureType>(&mut self) -> bool {
        if let Some(pos) = self.to_ref().iter().position(|id| *id == F2::IDENTIFIER) {
            if pos + 1 >= self.to_ref().len() {
                self.insert::<F1>()
            } else {
                self.insert_at::<F1>(pos + 1)
            }
        } else {
            false
        }
    }

    /// Removes the specified feature type from the set,
    /// shifting all elements after it to the left.
    ///
    /// Returns `true` if the feature type was present in the set.
    #[cfg(feature = "alloc")]
    pub fn remove<F: FeatureType>(&mut self) -> bool {
        if let Some(pos) = self.to_ref().iter().position(|id| *id == F::IDENTIFIER) {
            // SAFETY: Features are only being removed, not added.
            unsafe { self.to_mut().remove(pos) };
            true
        } else {
            false
        }
    }

    /// Get an immutable reference to underlying storage.
    #[must_use]
    pub const fn to_ref(&self) -> &[Identifier<'static>] {
        match self.inner {
            #[cfg(feature = "alloc")]
            FeatureSetInner::Runtime(ref vec) => vec.as_slice(),
            FeatureSetInner::Static(slice) => slice,
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
    pub unsafe fn to_mut(&mut self) -> &mut Vec<Identifier<'static>> {
        match self.inner {
            FeatureSetInner::Runtime(ref mut vec) => vec,
            FeatureSetInner::Static(slice) => {
                self.inner = FeatureSetInner::Runtime(Vec::from(slice));
                match self.inner {
                    FeatureSetInner::Runtime(ref mut vec) => vec,
                    FeatureSetInner::Static(_) => unreachable!(),
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all feature types.
pub trait FeatureType: 'static {
    /// The identifier of this feature type.
    const IDENTIFIER: Identifier<'static>;
}
