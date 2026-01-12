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

/// A set of biome features.
#[repr(transparent)]
#[derive(Debug)]
pub struct BiomeFeatureSet {
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    storage: RwLock<BiomeFeatures>,
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    storage: BiomeFeatures,
}

impl BiomeFeatureSet {
    /// Create an empty [`BiomeFeatureSet`] instance.
    #[must_use]
    pub const fn empty() -> Self { Self::new_static([&[]; 11]) }

    /// Create a new static [`BiomeFeatureSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided slice contains duplicate entries.
    #[must_use]
    pub const fn new_static(features: [&'static [Identifier<'static>]; 11]) -> Self {
        Self {
            #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
            storage: RwLock::new(BiomeFeatures::from_arrays(features)),
            #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
            storage: BiomeFeatures::from_arrays(features),
        }
    }

    /// Create a new runtime-allocated [`BiomeFeatureSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided vector contains duplicate entries.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new_runtime(vec: [Vec<Identifier<'static>>; 11]) -> Self {
        Self {
            #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
            storage: RwLock::new(BiomeFeatures::from_vectors(vec)),
            #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
            storage: BiomeFeatures::from_vectors(vec),
        }
    }

    /// Acquire a reference without blocking the current thread.
    #[inline]
    #[must_use]
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    pub const fn read(&self) -> &BiomeFeatures { &self.storage }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn read(&self) -> async_lock::RwLockReadGuard<'_, BiomeFeatures> {
        self.storage.read_blocking()
    }

    /// Acquire a read lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, BiomeFeatures> { self.storage.read() }

    /// Acquire a read lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, BiomeFeatures> {
        self.storage.read().expect("RwLock was poisoned!")
    }

    /// Acquire a read lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn read_async(&self) -> async_lock::RwLockReadGuard<'_, BiomeFeatures> {
        self.storage.read().await
    }

    /// Acquire a mutable reference without blocking the current thread.
    #[inline]
    #[must_use]
    #[cfg(not(any(feature = "async", feature = "parking_lot", feature = "std")))]
    pub const fn write(&mut self) -> &mut BiomeFeatures { &mut self.storage }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(feature = "async", feature = "std"))]
    pub fn write(&self) -> async_lock::RwLockWriteGuard<'_, BiomeFeatures> {
        self.storage.write_blocking()
    }

    /// Acquire a write lock, blocking the current thread.
    #[inline]
    #[cfg(all(not(feature = "async"), feature = "parking_lot"))]
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, BiomeFeatures> { self.storage.write() }

    /// Acquire a write lock, blocking the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the [`RwLock`] was poisoned.
    #[inline]
    #[cfg(all(not(feature = "async"), not(feature = "parking_lot"), feature = "std"))]
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, BiomeFeatures> {
        self.storage.write().expect("RwLock was poisoned!")
    }

    /// Acquire a write lock asynchronously.
    #[inline]
    #[cfg(feature = "async")]
    pub async fn write_async(&self) -> async_lock::RwLockWriteGuard<'_, BiomeFeatures> {
        self.storage.write().await
    }
}

// -------------------------------------------------------------------------------------------------

/// A collection of all biome feature generator sets.
#[derive(Debug)]
#[expect(missing_docs, reason = "TODO: Needs documentation")]
pub struct BiomeFeatures {
    pub raw_generation: BiomeFeatureStorage,
    pub lakes: BiomeFeatureStorage,
    pub local_modifications: BiomeFeatureStorage,
    pub underground_structures: BiomeFeatureStorage,
    pub surface_structures: BiomeFeatureStorage,
    pub strongholds: BiomeFeatureStorage,
    pub underground_ores: BiomeFeatureStorage,
    pub underground_decoration: BiomeFeatureStorage,
    pub fluid_springs: BiomeFeatureStorage,
    pub vegetal_decoration: BiomeFeatureStorage,
    pub top_layer_modification: BiomeFeatureStorage,
}

impl BiomeFeatures {
    /// Create a new [`BiomeFeatures`] from the provided set of feature arrays.
    ///
    /// # Panics
    ///
    /// Panics if any of the provided arrays contain duplicate entries.
    #[must_use]
    #[expect(clippy::many_single_char_names, reason = "Array deconstruction")]
    pub const fn from_arrays(features: [&'static [Identifier<'static>]; 11]) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k] = features;
        Self {
            raw_generation: BiomeFeatureStorage::new_static(a),
            lakes: BiomeFeatureStorage::new_static(b),
            local_modifications: BiomeFeatureStorage::new_static(c),
            underground_structures: BiomeFeatureStorage::new_static(d),
            surface_structures: BiomeFeatureStorage::new_static(e),
            strongholds: BiomeFeatureStorage::new_static(f),
            underground_ores: BiomeFeatureStorage::new_static(g),
            underground_decoration: BiomeFeatureStorage::new_static(h),
            fluid_springs: BiomeFeatureStorage::new_static(i),
            vegetal_decoration: BiomeFeatureStorage::new_static(j),
            top_layer_modification: BiomeFeatureStorage::new_static(k),
        }
    }

    /// Create a new [`BiomeFeatures`] from the provided set of feature vectors.
    ///
    /// # Panics
    ///
    /// Panics if any of the provided vectors contain duplicate entries.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[expect(clippy::many_single_char_names, reason = "Array deconstruction")]
    pub fn from_vectors(features: [Vec<Identifier<'static>>; 11]) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k] = features;
        Self {
            raw_generation: BiomeFeatureStorage::new_runtime(a),
            lakes: BiomeFeatureStorage::new_runtime(b),
            local_modifications: BiomeFeatureStorage::new_runtime(c),
            underground_structures: BiomeFeatureStorage::new_runtime(d),
            surface_structures: BiomeFeatureStorage::new_runtime(e),
            strongholds: BiomeFeatureStorage::new_runtime(f),
            underground_ores: BiomeFeatureStorage::new_runtime(g),
            underground_decoration: BiomeFeatureStorage::new_runtime(h),
            fluid_springs: BiomeFeatureStorage::new_runtime(i),
            vegetal_decoration: BiomeFeatureStorage::new_runtime(j),
            top_layer_modification: BiomeFeatureStorage::new_runtime(k),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The underlying storage for a [`BiomeFeatureSet`].
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct BiomeFeatureStorage {
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

impl BiomeFeatureStorage {
    /// Create a new static [`BiomeFeatureSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided slice contains duplicate entries.
    #[must_use]
    pub const fn new_static(slice: &'static [Identifier<'static>]) -> Self {
        assert_no_duplicates(slice);
        Self { inner: FeatureSetInner::Static(slice) }
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
        Self { inner: FeatureSetInner::Runtime(vec) }
    }

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

/// Asserts that the given slice contains no duplicate entries.
const fn assert_no_duplicates(slice: &[Identifier<'static>]) {
    let mut i = 0;
    while i < slice.len() {
        let mut j = i + 1;
        while j < slice.len() {
            assert!(
                !slice[i].const_eq(&slice[j]),
                "`BiomeFeatureStorage` contains duplicate entries!"
            );
            j += 1;
        }
        i += 1;
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all feature types.
pub trait FeatureType: 'static {
    /// The identifier of this feature type.
    const IDENTIFIER: Identifier<'static>;
}
