//! TODO

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    entry::{IndexedEntry, IndexedValue},
    index::EntryIndex,
    reference::IndexedReference,
};

mod iter;
pub use iter::CompoundIter;

/// An NBT Compound that is indexed by an [`IndexCore`].
pub struct IndexedCompound<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
    core: A::CORE<'data, C>,
    index: usize,
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedCompound<'data, A, C> {
    /// Create a new [`IndexedCompound`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: A::CORE<'data, C>, index: usize) -> Self { Self { core, index } }

    #[inline]
    #[must_use]
    fn entries(&self) -> &[EntryIndex] {
        // SAFETY: `IndexedCompound` guarantees that `self.index` is a valid index.
        unsafe { self.core.entry_range(self.index) }
    }

    /// Get the number of entries in this compound.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.entries().len() }

    /// Check if this compound is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.entries().is_empty() }
}

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedCompound<'data, A, C> {
    /// Return a reference to the stored value for `key`, if it is present, else
    /// `None`.
    #[must_use]
    pub fn get<'a, K: PartialEq<MStr> + ?Sized>(
        &'a self,
        key: &K,
    ) -> Option<IndexedValue<'a, Ref, C>> {
        for entry in self.entries() {
            // SAFETY: `IndexedCompound` guarantees that `entry.name()` is a valid index.
            let entry_key = unsafe {
                IndexedReference::<_, Ref>::new(<C as IndexCore<A>>::root(&self.core), entry.name())
            };

            if key == entry_key.get() {
                // SAFETY: `IndexedCompound` guarantees that `entry.value()` is a valid index.
                return Some(unsafe { IndexedValue::<Ref, C>::new(&self.core, entry.value()) });
            }
        }
        None
    }

    /// Return the stored value for `key`, if it is present, else `None`.
    #[must_use]
    pub fn into_entry<K: PartialEq<MStr> + ?Sized>(
        self,
        key: &K,
    ) -> Option<IndexedEntry<'data, Ref, C>> {
        for entry in self.entries() {
            // SAFETY: `IndexedCompound` guarantees that `entry.name()` is a valid index.
            let entry_key = unsafe {
                IndexedReference::<_, Ref>::new(<C as IndexCore<A>>::root(&self.core), entry.name())
            };

            if key == entry_key.get() {
                let entry = *entry;
                let core = A::into_core(self.core);

                // SAFETY: `IndexedCompound` guarantees that `entry.value()` is a valid index.
                return Some(unsafe { IndexedEntry::<Ref, C>::new(core, entry) });
            }
        }
        None
    }

    /// Get a key-value pair by index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<IndexedEntry<'_, Ref, C>> {
        self.entries().get(index).copied().map(|entry| {
            // SAFETY: `IndexedCompound` guarantees that `entry` has valid indexes.
            unsafe { IndexedEntry::<Ref, C>::new(&self.core, entry) }
        })
    }

    /// Return an iterator over the entries in this compound.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> CompoundIter<'_, 'data, A, C> { CompoundIter::new(self) }
}

// -------------------------------------------------------------------------------------------------

impl<'data, C: IndexCore<Mut> + 'data> IndexedCompound<'data, Mut, C> {
    /// Return a mutable reference to the stored value for `key`, if it is
    /// present, else `None`.
    #[must_use]
    pub fn get_mut<'a, K: PartialEq<MStr> + ?Sized>(
        &'a mut self,
        key: &K,
    ) -> Option<IndexedValue<'a, Mut, C>> {
        for entry in self.entries() {
            // SAFETY: `IndexedCompound` guarantees that `entry.name()` is a valid index.
            let entry_key = unsafe {
                IndexedReference::<_, Ref>::new(
                    <C as IndexCore<Mut>>::root(self.core),
                    entry.name(),
                )
            };

            if key == entry_key.get() {
                // SAFETY: `IndexedCompound` guarantees that `entry.value()` is a valid index.
                let value = entry.value();
                return Some(unsafe { IndexedValue::<Mut, C>::new(self.core, value) });
            }
        }
        None
    }

    /// Get a key-value pair by index.
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<IndexedEntry<'_, Mut, C>> {
        self.entries().get(index).copied().map(|entry| {
            // SAFETY: `IndexedCompound` guarantees that `entry` has valid indexes.
            unsafe { IndexedEntry::<Mut, C>::new(self.core, entry) }
        })
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, A: NbtAccess, C: IndexCore<A>> Clone for IndexedCompound<'a, A, C>
where
    A::CORE<'a, C>: Clone,
{
    fn clone(&self) -> Self { Self { core: self.core.clone(), index: self.index } }
}
