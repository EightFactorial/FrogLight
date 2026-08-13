use alloc::{borrow::Cow, boxed::Box};
use core::range::Range;

use crate::types::indexed::{
    IndexedSnbt,
    core::{IndexCore, IndexedSnbtSlice, SliceCore},
    entry::EntryIndex,
};

/// TODO
pub struct CowCore<'core> {
    pub(crate) root: Cow<'core, str>,
    pub(crate) entries: Box<[EntryIndex]>,
}

impl IndexCore for CowCore<'_> {
    type RootLong<'a>
        = &'a str
    where
        Self: 'a;

    #[inline]
    fn root(&self) -> &str { self.root.as_ref() }

    #[inline]
    fn root_long(&self) -> Self::RootLong<'_> { self.root.as_ref() }

    #[inline]
    unsafe fn get_entries(&self, range: Range<usize>) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { self.entries.get_unchecked(range) }
    }
}

impl<'core> CowCore<'core> {
    /// Create a new [`CowCore`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the entry list is valid for the root string.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'core str, entries: Box<[EntryIndex]>) -> Self {
        Self { root: Cow::Borrowed(root), entries }
    }

    /// Create a [`CowCore`] from a [`SliceCore`].
    ///
    /// This does not modify the contents.
    #[inline]
    #[must_use]
    pub fn from_slice(core: SliceCore<'core>) -> Self {
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
    pub const fn entries(&self) -> &[EntryIndex] { &self.entries }

    /// Temporarily use this [`CowCore`] as a [`SliceCore`].
    #[must_use]
    pub fn as_slice_for<R>(mut self, f: impl FnOnce(&IndexedSnbtSlice<'_>) -> R) -> (Self, R) {
        let slice = unsafe { SliceCore::<'_>::new(&self.root, self.entries) };
        let snbt = IndexedSnbt::new(slice);
        let result = (f)(&snbt);

        let slice = snbt.core;
        self.entries = slice.entries;
        (self, result)
    }

    /// Create an owned [`CowCore`] by cloning it's data.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> CowCore<'static> {
        CowCore { root: Cow::Owned(self.root.into_owned()), entries: self.entries }
    }
}
