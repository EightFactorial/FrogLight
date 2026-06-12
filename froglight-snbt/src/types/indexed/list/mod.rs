//! TODO

use core::{fmt, range::Range};

use crate::types::indexed::{core::IndexCore, entry::EntryIndex, reference::ValueReference};

mod iter;
pub use iter::{ListIter, SliceIter};

mod slice;
pub use slice::IndexedSlice;

/// A list of values indexed by an [`IndexCore`].
pub struct IndexedList<'data, C: IndexCore> {
    core: &'data C,
    range: Range<usize>,
}

impl<'data, C: IndexCore> IndexedList<'data, C> {
    /// Create a new [`IndexedList`] with the given core and range.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the range is valid for the core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: &'data C, range: Range<usize>) -> Self { Self { core, range } }

    /// Get the [`EntryIndexes`](EntryIndex) of this list.
    #[inline]
    #[must_use]
    pub(crate) fn entries(&self) -> &[EntryIndex] {
        // SAFETY: `IndexedList` guarantees that this is safe.
        unsafe { self.core.get_entries(self.range) }
    }

    /// Get a value by it's index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<ValueReference<'data, C>> {
        self.entries()
            .get(index)
            .map(|entry| unsafe { ValueReference::new(self.core, entry.value()) })
    }

    /// Create an iterator over this list.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> ListIter<'_, 'data, C> { ListIter::new(self) }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore> fmt::Debug for IndexedList<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<C: IndexCore> Clone for IndexedList<'_, C> {
    #[inline]
    fn clone(&self) -> Self { *self }
}
impl<C: IndexCore> Copy for IndexedList<'_, C> {}

impl<C: IndexCore> PartialEq for IndexedList<'_, C> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.core.root() == other.core.root()
    }
}
impl<C: IndexCore> Eq for IndexedList<'_, C> {}
