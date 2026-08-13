//! TODO

use core::fmt;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    entry::{IndexedEntry, IndexedValue},
    index::EntryIndex,
    reference::IndexedReference,
};

mod iter;
pub use iter::{CompoundIter, CompoundOwnedIter};

/// An NBT Compound that is indexed by an [`IndexCore`].
pub struct IndexedCompound<'index, A: NbtAccess, C: IndexCore<A> + 'index> {
    core: A::CORE<'index, C>,
    index: usize,
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> IndexedCompound<'index, A, C> {
    /// Create a new [`IndexedCompound`] from the given core and range index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: A::CORE<'index, C>, index: usize) -> Self { Self { core, index } }

    /// Get the list of entries held by this compound.
    #[inline]
    #[must_use]
    fn entries(&self) -> &[EntryIndex] {
        // SAFETY: `IndexedCompound` guarantees that `index` is a valid range index.
        unsafe { self.core.entry_range(self.index) }
    }

    /// Get the index of the entry with the given key, if it exists.
    #[must_use]
    fn entry_with_key<K: PartialEq<MStr> + ?Sized>(&self, key: &K) -> Option<&EntryIndex> {
        self.entries().iter().find(|entry| {
            let name = entry.name();
            let root = <C as IndexCore<A>>::root(&self.core);

            // SAFETY: `IndexedCompound` guarantees that `name` is a valid index.
            let entry_key = unsafe { IndexedReference::<_, Ref>::new(root, name) };

            key == entry_key.get()
        })
    }

    /// Get the number of entries in this compound.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.entries().len() }

    /// Check if this compound is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.entries().is_empty() }

    /// Return the stored value for `key`, if it is present, else `None`.
    #[must_use]
    pub fn get<K: PartialEq<MStr> + ?Sized>(self, key: &K) -> Option<IndexedValue<'index, A, C>> {
        if let Some(index) = self.entry_with_key(key).copied() {
            let value = index.value();
            // SAFETY: `IndexedCompound` guarantees that `value` is a valid index.
            Some(unsafe { IndexedValue::<A, C>::new(self.core, value) })
        } else {
            None
        }
    }

    /// Get a key-value pair by index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_index(self, index: usize) -> Option<IndexedEntry<'index, A, C>> {
        if let Some(entry) = self.entries().get(index).copied() {
            // SAFETY: `IndexedCompound` guarantees that `entry` has valid indexes.
            Some(unsafe { IndexedEntry::<A, C>::new(self.core, entry) })
        } else {
            None
        }
    }

    /// Return a reference to the stored value for `key`, if it is present, else
    /// `None`.
    #[must_use]
    pub fn get_ref<'a, K: PartialEq<MStr> + ?Sized>(
        &'a self,
        key: &K,
    ) -> Option<IndexedValue<'a, Ref, C>>
    where
        C: IndexCore<Ref>,
    {
        if let Some(index) = self.entry_with_key(key).copied() {
            let value = index.value();
            // SAFETY: `IndexedCompound` guarantees that `value` is a valid index.
            Some(unsafe { IndexedValue::<Ref, C>::new(&self.core, value) })
        } else {
            None
        }
    }

    /// Get a reference to a key-value pair by index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_index_ref(&self, index: usize) -> Option<IndexedEntry<'_, Ref, C>>
    where
        C: IndexCore<Ref>,
    {
        if let Some(entry) = self.entries().get(index).copied() {
            // SAFETY: `IndexedCompound` guarantees that `entry` has valid indexes.
            Some(unsafe { IndexedEntry::<Ref, C>::new(&self.core, entry) })
        } else {
            None
        }
    }

    /// Return an iterator over the entries in this compound.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> CompoundIter<'_, 'index, A, C>
    where
        C: IndexCore<Ref>,
    {
        CompoundIter::new(self)
    }

    /// Return an owned iterator over the entries in this compound.
    ///
    /// # Note
    ///
    /// This function requires `Copy`, but the iterator only requires `Clone`!
    ///
    /// This is to prevent accidental `Clone`s, which can be very expensive.
    #[inline]
    #[must_use]
    pub const fn into_iter(self) -> CompoundOwnedIter<'index, A, C>
    where
        <A as NbtAccess>::CORE<'index, C>: Copy,
    {
        CompoundOwnedIter::new(self)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'index, C: IndexCore<Mut> + 'index> IndexedCompound<'index, Mut, C> {
    /// Return a mutable reference to the stored value for `key`, if it is
    /// present, else `None`.
    #[must_use]
    pub fn get_mut<'a, K: PartialEq<MStr> + ?Sized>(
        &'a mut self,
        key: &K,
    ) -> Option<IndexedValue<'a, Mut, C>> {
        if let Some(index) = self.entry_with_key(key).copied() {
            let value = index.value();
            // SAFETY: `IndexedCompound` guarantees that `value` is a valid index.
            Some(unsafe { IndexedValue::<Mut, C>::new(self.core, value) })
        } else {
            None
        }
    }

    /// Get a key-value pair by index.
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<IndexedEntry<'_, Mut, C>> {
        if let Some(entry) = self.entries().get(index).copied() {
            // SAFETY: `IndexedCompound` guarantees that `entry` has valid indexes.
            Some(unsafe { IndexedEntry::<Mut, C>::new(self.core, entry) })
        } else {
            None
        }
    }

    /// Return an iterator over the entries in this compound.
    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> CompoundIter<'_, 'index, Mut, C> { CompoundIter::new(self) }
}

// -------------------------------------------------------------------------------------------------

impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug for IndexedCompound<'_, A, C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter().map(IndexedEntry::pair)).finish()
    }
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Clone for IndexedCompound<'index, A, C>
where
    <A as NbtAccess>::CORE<'index, C>: Clone,
{
    #[inline]
    fn clone(&self) -> Self { Self { core: self.core.clone(), index: self.index } }
}
impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Copy for IndexedCompound<'index, A, C> where
    <A as NbtAccess>::CORE<'index, C>: Copy
{
}

impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> PartialEq for IndexedCompound<'index, A, C> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && <C as IndexCore<A>>::root(&self.core) == <C as IndexCore<A>>::root(&other.core)
    }
}
impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Eq for IndexedCompound<'index, A, C> {}
