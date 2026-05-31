use alloc::vec::Vec;
use core::range::Range;

use crate::types::indexed::core::IndexCore;

/// TODO
pub struct SliceCore<'data> {
    pub(super) root: &'data str,
    pub(super) entries: Vec<()>,
}

impl IndexCore for SliceCore<'_> {
    #[inline]
    fn root(&self) -> &str { self.root }

    #[inline]
    unsafe fn get_entries(&self, range: Range<usize>) -> &[()] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { self.entries.get_unchecked(range) }
    }
}

impl<'data> SliceCore<'data> {
    /// Create a new [`SliceCore`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the entry list is valid for the root string.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'data str, entries: Vec<()>) -> Self { Self { root, entries } }

    /// Get the root string of this [`SliceCore`].
    #[inline]
    #[must_use]
    pub const fn root(&self) -> &'data str { self.root }

    /// Get a slice of the entries in this [`SliceCore`].
    #[inline]
    #[must_use]
    pub const fn entries(&self) -> &[()] { self.entries.as_slice() }
}
