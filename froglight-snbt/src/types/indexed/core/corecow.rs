use alloc::{borrow::Cow, vec::Vec};
use core::range::Range;

use crate::types::indexed::core::{IndexCore, SliceCore};

/// TODO
pub struct CowCore<'data> {
    root: Cow<'data, str>,
    entries: Vec<()>,
}

impl IndexCore for CowCore<'_> {
    #[inline]
    fn root(&self) -> &str { self.root.as_ref() }

    #[inline]
    unsafe fn get_entries(&self, range: Range<usize>) -> &[()] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { self.entries.get_unchecked(range) }
    }
}

impl<'data> CowCore<'data> {
    /// Create a new [`CowCore`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the entry list is valid for the root string.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'data str, entries: Vec<()>) -> Self {
        Self { root: Cow::Borrowed(root), entries }
    }

    /// Create a [`CowCore`] from a [`SliceCore`].
    ///
    /// This does not modify the contents.
    #[inline]
    #[must_use]
    pub fn from_slice(core: SliceCore<'data>) -> Self {
        Self { root: Cow::Borrowed(core.root), entries: core.entries }
    }

    /// Get the root string of this [`CowCore`].
    #[inline]
    #[must_use]
    pub const fn root(&self) -> &str {
        match self.root {
            Cow::Borrowed(s) => s,
            Cow::Owned(ref s) => s.as_str(),
        }
    }

    /// Get a slice of the entries in this [`CowCore`].
    #[inline]
    #[must_use]
    pub const fn entries(&self) -> &[()] { self.entries.as_slice() }

    /// Create an owned [`CowCore`] by cloning it's data.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> CowCore<'static> {
        CowCore { root: Cow::Owned(self.root.into_owned()), entries: self.entries }
    }
}
