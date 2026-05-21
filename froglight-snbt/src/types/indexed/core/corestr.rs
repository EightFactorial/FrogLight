use alloc::vec::Vec;
use core::range::Range;

use crate::types::indexed::{core::IndexCore, index::EntryIndex};

pub struct StrCore<'data> {
    pub(super) data: &'data str,
    pub(super) entries: Vec<EntryIndex>,
    pub(super) ranges: Vec<Range<usize>>,
}

impl<'data> StrCore<'data> {
    /// Creates a new [`StrCore`] from the given data and entries.
    ///
    /// # SAFETY
    ///
    /// TODO
    #[inline]
    #[must_use]
    pub const unsafe fn new(
        data: &'data str,
        entries: Vec<EntryIndex>,
        ranges: Vec<Range<usize>>,
    ) -> Self {
        Self { data, entries, ranges }
    }
}

impl IndexCore for StrCore<'_> {
    fn root(&self) -> &str { self.data }

    fn entries(&self) -> &[EntryIndex] { self.entries.as_slice() }

    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe {
            let range = self.ranges.get_unchecked(index);
            self.entries.as_slice().get_unchecked(*range)
        }
    }
}
