//! TODO

use core::{fmt, range::Range};

use crate::types::indexed::{
    core::IndexCore,
    entry::EntryIndex,
    reference::{EntryReference, ValueReference},
};

mod iter;
use iter::CompoundIter;

/// An SNBT Compound that is indexed by an [`IndexCore`].
pub struct IndexedCompound<'data, C: IndexCore> {
    core: &'data C,
    range: Range<usize>,
}

impl<'data, C: IndexCore> IndexedCompound<'data, C> {
    /// Create a new [`IndexedCompound`] from a range and core.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the entry range is valid for the provided
    /// core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: &'data C, range: Range<usize>) -> Self { Self { core, range } }

    /// Get the [`EntryIndexes`](EntryIndex) of this compound.
    #[inline]
    #[must_use]
    pub(crate) fn entries(&self) -> &[EntryIndex] {
        // SAFETY: `IndexedCompound` guarantees that this is safe.
        unsafe { self.core.get_entries(self.range) }
    }

    /// Get the number of entries in this compound.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.entries().len() }

    /// Check if this compound is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.entries().is_empty() }

    /// Return a reference to the value matching the `key`, if it is present,
    /// else `None`.
    #[must_use]
    pub fn get<K: PartialEq<str> + ?Sized>(&self, key: &K) -> Option<ValueReference<'data, C>> {
        self.entries()
            .iter()
            .find(|e| key == unsafe { e.name().read_value(self.core.root()) }.as_ref())
            .map(|e| unsafe { ValueReference::new(self.core, e.value()) })
    }

    /// Get a key-value pair by index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<EntryReference<'data, C>> {
        self.entries().get(index).map(|e| unsafe { EntryReference::new(self.core, *e) })
    }

    /// Create an iterator over this compound.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> CompoundIter<'_, 'data, C> { CompoundIter::new(self) }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore> fmt::Debug for IndexedCompound<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter().map(|e| (e.name(), e.value()))).finish()
    }
}

impl<C: IndexCore> Clone for IndexedCompound<'_, C> {
    fn clone(&self) -> Self { *self }
}
impl<C: IndexCore> Copy for IndexedCompound<'_, C> {}

impl<C: IndexCore> PartialEq for IndexedCompound<'_, C> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.core.root() == other.core.root()
    }
}
impl<C: IndexCore> Eq for IndexedCompound<'_, C> {}
