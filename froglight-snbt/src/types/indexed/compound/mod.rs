//! TODO

use core::{fmt, range::Range};

use crate::types::indexed::{core::IndexCore, entry::EntryIndex, reference::EntryReference};

mod iter;

/// An SNBT Compound that is indexed by an [`IndexCore`].
pub struct IndexedCompound<'data, C: IndexCore> {
    range: Range<usize>,
    core: &'data C,
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
    pub const unsafe fn new(range: Range<usize>, core: &'data C) -> Self { Self { range, core } }

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

    /// Return a reference to the stored value for `key`, if it is present, else
    /// `None`.
    #[must_use]
    pub fn get<K: PartialEq<str> + ?Sized>(&self, key: &K) -> Option<EntryReference<'data, C>> {
        self.entries()
            .iter()
            .find(|e| key == unsafe { e.name().read_value(self.core.root()) }.as_ref())
            .map(|e| unsafe { EntryReference::new(*e, self.core) })
    }

    /// Get a key-value pair by index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<EntryReference<'data, C>> {
        self.entries().get(index).map(|e| unsafe { EntryReference::new(*e, self.core) })
    }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore> fmt::Debug for IndexedCompound<'_, C> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_map().entries(self.iter()).finish()
        todo!();
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
