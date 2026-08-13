use alloc::boxed::Box;
use core::range::Range;

use crate::types::indexed::{core::IndexCore, entry::EntryIndex};

/// TODO
pub struct SliceCore<'core> {
    pub(crate) root: &'core str,
    pub(crate) entries: Box<[EntryIndex]>,
}

impl<'core> IndexCore for SliceCore<'core> {
    type RootLong<'a>
        = &'core str
    where
        Self: 'a;

    #[inline]
    fn root(&self) -> &str { self.root }

    #[inline]
    fn root_long(&self) -> Self::RootLong<'_> { self.root }

    #[inline]
    unsafe fn get_entries(&self, range: Range<usize>) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { self.entries.get_unchecked(range) }
    }
}

impl<'core> SliceCore<'core> {
    /// Create a new [`SliceCore`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the entry list is valid for the root string.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'core str, entries: Box<[EntryIndex]>) -> Self {
        Self { root, entries }
    }

    /// Get the root string of this [`SliceCore`].
    #[inline]
    #[must_use]
    pub const fn root(&self) -> &'core str { self.root }

    /// Get a slice of the entries in this [`SliceCore`].
    #[inline]
    #[must_use]
    pub const fn entries(&self) -> &[EntryIndex] { &self.entries }
}
