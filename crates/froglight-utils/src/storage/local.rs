use core::{any::TypeId, fmt::Debug, marker::PhantomData};

use bevy_platform::hash::NoOpHash;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use downcast_rs::Downcast;
use indexmap::IndexMap;

use super::StorageWrapper;

/// A run-time modifiable storage for static values
/// accessible by either a [`TypeId`] or an index.
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone))]
pub struct IndexedLocalStorage<V: ?Sized + 'static, I: From<usize> + Into<usize>>(
    IndexMap<TypeId, StorageWrapper<V>, NoOpHash>,
    #[cfg_attr(feature = "reflect", reflect(ignore))] PhantomData<I>,
);

impl<V: ?Sized + 'static, I: From<usize> + Into<usize>> IndexedLocalStorage<V, I> {
    /// Create a new [`IndexedLocalStorage`].
    #[must_use]
    pub const fn new() -> Self { Self(IndexMap::with_hasher(NoOpHash), PhantomData) }

    /// Create a new [`IndexedLocalStorage`] with capacity for `n` values.
    #[must_use]
    pub fn with_capacity(n: usize) -> Self {
        Self(IndexMap::with_capacity_and_hasher(n, NoOpHash), PhantomData)
    }

    /// Get a stored value by its [`TypeId`].
    #[inline]
    #[must_use]
    pub fn get(&self, type_id: &TypeId) -> Option<StorageWrapper<V>> {
        self.0.get(type_id).copied()
    }

    /// Get a stored value by its index.
    #[inline]
    #[must_use]
    pub fn get_index(&self, key: I) -> Option<StorageWrapper<V>> {
        self.0.get_index(key.into()).map(|(_, v)| *v)
    }

    /// Get the index of a stored value.
    #[inline]
    #[must_use]
    pub fn get_index_of(&self, type_id: &TypeId) -> Option<I> {
        self.0.get_index_of(type_id).map(I::from)
    }

    /// Store a value using a [`TypeId`] as the key.
    ///
    /// Requires a function that returns a static reference to the value.
    #[inline]
    pub fn store(&mut self, type_id: TypeId, value: &'static V) {
        self.0.insert(type_id, StorageWrapper::new(value));
    }
}

// -------------------------------------------------------------------------------------------------
// Manual implementations for `IndexedLocalStorage` to avoid trait bounds

impl<V: Debug + ?Sized + 'static, I: From<usize> + Into<usize>> Debug
    for IndexedLocalStorage<V, I>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.0.fmt(f) }
}

impl<V: ?Sized + 'static, I: From<usize> + Into<usize>> Default for IndexedLocalStorage<V, I> {
    fn default() -> Self { Self::new() }
}

impl<V: ?Sized + 'static, I: From<usize> + Into<usize>> Clone for IndexedLocalStorage<V, I> {
    fn clone(&self) -> Self { Self(self.0.clone(), PhantomData) }
}

impl<V: Downcast + ?Sized + 'static, I: From<usize> + Into<usize>> PartialEq
    for IndexedLocalStorage<V, I>
{
    fn eq(&self, other: &Self) -> bool { IndexMap::eq(&self.0, &other.0) }
}
impl<V: Downcast + ?Sized + 'static, I: From<usize> + Into<usize>> Eq
    for IndexedLocalStorage<V, I>
{
}
