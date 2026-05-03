//! TODO

use alloc::borrow::Cow;
use core::range::Range;

use super::IndexedEntry;

/// The core of an [`IndexedNbtRef`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IndexedCoreRef<'data> {
    root: &'data [u8],
    entries: Cow<'data, [IndexedEntry]>,
    indexes: Cow<'data, [Range<usize>]>,
}
impl<'data> IndexedCoreRef<'data> {
    /// Create a new [`IndexedCoreRef`] from the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given list of entries and indexes
    /// is valid for the given root slice.
    #[inline]
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data [u8],
        entries: Cow<'data, [IndexedEntry]>,
        indexes: Cow<'data, [Range<usize>]>,
    ) -> Self {
        Self { root, entries, indexes }
    }

    /// Get the core byte slice.
    #[inline]
    #[must_use]
    pub(super) const fn root(&self) -> &'data [u8] { self.root }

    /// Get the core list of [`IndexedEntry`]s.
    #[must_use]
    pub(super) const fn entries(&self) -> &[IndexedEntry] {
        match self.entries {
            Cow::Borrowed(e) => e,
            Cow::Owned(ref e) => e.as_slice(),
        }
    }

    /// Get the core list of index ranges.
    #[must_use]
    pub(super) const fn indexes(&self) -> &[Range<usize>] {
        match self.indexes {
            Cow::Borrowed(i) => i,
            Cow::Owned(ref i) => i.as_slice(),
        }
    }

    /// Reborrow this [`IndexedCoreRef`] with a shorter lifetime.
    #[inline]
    #[must_use]
    pub(super) const fn reborrow(&self) -> IndexedCoreRef<'_> {
        IndexedCoreRef {
            root: self.root,
            entries: match self.entries {
                Cow::Borrowed(e) => Cow::Borrowed(e),
                Cow::Owned(ref e) => Cow::Borrowed(e.as_slice()),
            },
            indexes: match self.indexes {
                Cow::Borrowed(i) => Cow::Borrowed(i),
                Cow::Owned(ref i) => Cow::Borrowed(i.as_slice()),
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The core of an [`IndexedNbtMut`].
#[derive(Debug, PartialEq, Eq)]
pub(super) struct IndexedCoreMut<'data> {
    root: &'data mut [u8],
    entries: Cow<'data, [IndexedEntry]>,
    indexes: Cow<'data, [Range<usize>]>,
}

impl<'data> IndexedCoreMut<'data> {
    /// Create a new [`IndexedCoreMut`] from the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given list of entries and indexes
    /// is valid for the given root slice.
    #[inline]
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data mut [u8],
        entries: Cow<'data, [IndexedEntry]>,
        indexes: Cow<'data, [Range<usize>]>,
    ) -> Self {
        Self { root, entries, indexes }
    }

    /// Get the core byte slice.
    #[inline]
    #[must_use]
    pub(super) const fn root(&self) -> &[u8] { self.root }

    /// Get the core byte slice mutably.
    #[inline]
    #[must_use]
    pub(super) const fn root_mut(&mut self) -> &mut [u8] { self.root }

    /// Get the core list of [`IndexedEntry`]s.
    #[must_use]
    pub(super) const fn entries(&self) -> &[IndexedEntry] {
        match self.entries {
            Cow::Borrowed(e) => e,
            Cow::Owned(ref e) => e.as_slice(),
        }
    }

    /// Get the core list of index ranges.
    #[must_use]
    pub(super) const fn indexes(&self) -> &[Range<usize>] {
        match self.indexes {
            Cow::Borrowed(i) => i,
            Cow::Owned(ref i) => i.as_slice(),
        }
    }

    /// Reborrow this [`IndexedCoreMut`] with a shorter lifetime.
    #[inline]
    #[must_use]
    pub(super) const fn reborrow(&mut self) -> IndexedCoreMut<'_> {
        IndexedCoreMut {
            root: self.root,
            entries: match self.entries {
                Cow::Borrowed(e) => Cow::Borrowed(e),
                Cow::Owned(ref e) => Cow::Borrowed(e.as_slice()),
            },
            indexes: match self.indexes {
                Cow::Borrowed(i) => Cow::Borrowed(i),
                Cow::Owned(ref i) => Cow::Borrowed(i.as_slice()),
            },
        }
    }

    /// Get this [`IndexedCoreMut`] as a [`IndexedCoreRef`].
    #[inline]
    #[must_use]
    pub(super) const fn as_ref(&self) -> IndexedCoreRef<'_> {
        IndexedCoreRef {
            root: self.root,
            entries: match self.entries {
                Cow::Borrowed(e) => Cow::Borrowed(e),
                Cow::Owned(ref e) => Cow::Borrowed(e.as_slice()),
            },
            indexes: match self.indexes {
                Cow::Borrowed(i) => Cow::Borrowed(i),
                Cow::Owned(ref i) => Cow::Borrowed(i.as_slice()),
            },
        }
    }

    /// Convert this [`IndexedCoreMut`] into a [`IndexedCoreRef`].
    #[inline]
    #[must_use]
    pub(super) fn into_ref(self) -> IndexedCoreRef<'data> {
        IndexedCoreRef { root: self.root, entries: self.entries, indexes: self.indexes }
    }
}
