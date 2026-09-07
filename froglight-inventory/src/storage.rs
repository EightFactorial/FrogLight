//! TODO

use alloc::boxed::Box;
use core::{any::TypeId, fmt, hash};

use downcast_rs::Downcast;
use froglight_common::{
    crates::{
        foldhash::fast::FixedState,
        indexmap::map::{Entry, Iter, IterMut},
    },
    types::IndexMap,
};

/// A storage for arbitrary types that implement [`InventoryStorageType`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryStorage {
    storage: IndexMap<TypeId, Box<dyn InventoryStorageType>, FixedState>,
}

impl Default for InventoryStorage {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl InventoryStorage {
    /// Create a new, empty [`InventoryStorage`].
    #[must_use]
    pub const fn new() -> Self {
        Self { storage: IndexMap::with_hasher(FixedState::with_seed(0x04)) }
    }

    /// Try to initialize the [`InventoryStorageType`] of the given type.
    ///
    /// # Errors
    ///
    /// Returns the input if the storage already contains a value of that type.
    pub fn try_init<S: InventoryStorageType>(&mut self, value: S) -> Result<(), S> {
        match self.storage.entry(TypeId::of::<S>()) {
            Entry::Occupied(..) => Err(value),
            Entry::Vacant(entry) => {
                entry.insert(Box::new(value));
                Ok(())
            }
        }
    }

    /// Insert a value into the storage, returning the old value if it existed.
    pub fn insert<S: InventoryStorageType>(&mut self, value: S) -> Option<Box<S>> {
        self.insert_boxed(Box::new(value)).and_then(|old| old.into_any().downcast().ok())
    }

    /// Insert a value into the storage, returning the old value if it existed.
    pub fn insert_boxed(
        &mut self,
        value: Box<dyn InventoryStorageType>,
    ) -> Option<Box<dyn InventoryStorageType>> {
        self.storage.insert(value.as_any().type_id(), value)
    }

    /// Get a reference to the value of the given type, if it exists.
    #[must_use]
    pub fn get<S: InventoryStorageType>(&self) -> Option<&S> {
        self.storage.get(&TypeId::of::<S>()).and_then(|value| value.as_any().downcast_ref())
    }

    /// Get a mutable reference to the value of the given type, if it exists.
    #[must_use]
    pub fn get_mut<S: InventoryStorageType>(&mut self) -> Option<&mut S> {
        self.storage.get_mut(&TypeId::of::<S>()).and_then(|value| value.as_any_mut().downcast_mut())
    }

    /// Returns `true` if the storage contains a value of the given type.
    #[must_use]
    pub fn contains<S: InventoryStorageType>(&self) -> bool {
        self.storage.contains_key(&TypeId::of::<S>())
    }

    /// Remove the value of the given type from the storage, returning it if it
    /// existed.
    ///
    /// # Note
    ///
    /// This method uses [`IndexMap::swap_remove`] to remove the value,
    /// which changes the order of the remaining values.
    #[must_use]
    pub fn take_swap<S: InventoryStorageType>(&mut self) -> Option<Box<S>> {
        self.storage
            .swap_remove(&TypeId::of::<S>())
            .and_then(|value| value.into_any().downcast().ok())
    }

    /// Remove the value of the given type from the storage, returning it if it
    /// existed.
    ///
    /// # Note
    ///
    /// This method uses [`IndexMap::shift_remove`] to remove the value,
    /// which preserves the order of the remaining values.
    #[must_use]
    pub fn take_shift<S: InventoryStorageType>(&mut self) -> Option<Box<S>> {
        self.storage
            .shift_remove(&TypeId::of::<S>())
            .and_then(|value| value.into_any().downcast().ok())
    }

    /// Create an iterator over the values in the storage.
    #[inline]
    #[must_use]
    #[expect(clippy::iter_without_into_iter, reason = "Ignored")]
    pub fn iter(&self) -> Iter<'_, TypeId, Box<dyn InventoryStorageType>> { self.storage.iter() }

    /// Create a mutable iterator over the values in the storage.
    #[inline]
    #[must_use]
    #[expect(clippy::iter_without_into_iter, reason = "Ignored")]
    pub fn iter_mut(&mut self) -> IterMut<'_, TypeId, Box<dyn InventoryStorageType>> {
        self.storage.iter_mut()
    }
}

/// A trait for types that can be stored in an [`InventoryStorage`].
pub trait InventoryStorageType: fmt::Debug + Downcast + Send + Sync + 'static {
    /// A `dyn`-compatible method for [`Clone`].
    fn dyn_clone(&self) -> Box<dyn InventoryStorageType>;

    /// A `dyn`-compatible method for [`Eq`].
    fn dyn_eq(&self, other: &dyn InventoryStorageType) -> bool;

    /// A `dyn`-compatible method for [`Hash`].
    fn dyn_hash(&self, state: &mut dyn hash::Hasher);
}

// -------------------------------------------------------------------------------------------------

impl Clone for Box<dyn InventoryStorageType> {
    #[inline]
    fn clone(&self) -> Self { self.dyn_clone() }
}

impl PartialEq for dyn InventoryStorageType {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.dyn_eq(other) }
}
impl Eq for dyn InventoryStorageType {}

impl hash::Hash for InventoryStorage {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        for (key, value) in &self.storage {
            key.hash(state);
            value.dyn_hash(state);
        }
    }
}
