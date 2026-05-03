//! TODO

use core::range::Range;

use froglight_mutf8::prelude::MStr;

use crate::types::borrowed::{
    IndexedCoreMut, IndexedCoreRef,
    reference::{BorrowedIndex, BorrowedMut, BorrowedRef},
};

/// An NBT compound tag with a reference to its data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedCompoundRef<'data> {
    core: IndexedCoreRef<'data>,
    range: Range<usize>,
}

/// A reference to an entry in an [`IndexedCompound`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedEntryRef<'data> {
    root: &'data [u8],
    entry: IndexedEntry,
}

impl<'data> IndexedCompoundRef<'data> {
    /// Create a new [`IndexedCompoundRef`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `range` is a valid range of `core`.
    #[must_use]
    pub(super) const unsafe fn new(core: IndexedCoreRef<'data>, range: Range<usize>) -> Self {
        Self { core, range }
    }

    /// Get the number of entries in this [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.range.end.saturating_sub(self.range.start) }

    /// Returns `true` if the compound is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get a reference to the entry with the given key, if it exists.
    ///
    /// Returns `None` if no entry with the given key exists.
    #[must_use]
    pub fn get<'a, T: PartialEq<MStr> + ?Sized>(&'a self, key: &T) -> Option<BorrowedRef<'a, ()>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        for entry in entries {
            // SAFETY: The entry uses the same `root` so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.core.root(), entry.name) };

            if key == key_ref.get_ref() {
                // SAFETY: The ref uses the same `root` so the index is valid.
                return Some(unsafe { BorrowedRef::new(self.core.root(), entry.contents) });
            }
        }
        None
    }

    /// Get a reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<BorrowedEntryRef<'_>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        entries.get(index).copied().map(|entry| BorrowedEntryRef { root: self.core.root(), entry })
    }

    /// Get an iterator over all entries in this compound.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedEntryRef<'_>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        entries.iter().copied().map(|entry| BorrowedEntryRef { root: self.core.root(), entry })
    }

    /// Reborrow this [`IndexedCompoundRef`] with a shorter lifetime.
    #[inline]
    #[must_use]
    pub const fn reborrow(&self) -> IndexedCompoundRef<'_> {
        IndexedCompoundRef { core: self.core.reborrow(), range: self.range }
    }
}

impl<'data> BorrowedEntryRef<'data> {
    /// Get a reference to the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name(self) -> BorrowedRef<'data, MStr> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedRef::new(self.root, self.entry.name) }
    }

    /// Get a reference to the contents of this entry.
    #[inline]
    #[must_use]
    pub const fn contents(self) -> BorrowedRef<'data, ()> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedRef::new(self.root, self.entry.contents) }
    }
}

// -------------------------------------------------------------------------------------------------

/// An NBT compound tag with a mutable reference to its data.
#[derive(Debug, PartialEq, Eq)]
pub struct IndexedCompoundMut<'data> {
    core: IndexedCoreMut<'data>,
    range: Range<usize>,
}

/// A mutable reference to an entry in an [`IndexedCompound`].
#[derive(Debug, PartialEq, Eq)]
pub struct BorrowedEntryMut<'data> {
    root: &'data mut [u8],
    entry: IndexedEntry,
}

impl<'data> IndexedCompoundMut<'data> {
    /// Create a new [`IndexedCompound`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `range` is a valid range of `core`.
    #[must_use]
    pub(super) const unsafe fn new(core: IndexedCoreMut<'data>, range: Range<usize>) -> Self {
        Self { core, range }
    }

    /// Get the number of entries in this [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.range.end.saturating_sub(self.range.start) }

    /// Returns `true` if the compound is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get a reference to the entry with the given key, if it exists.
    ///
    /// Returns `None` if no entry with the given key exists.
    #[must_use]
    pub fn get<'a, T: PartialEq<MStr> + ?Sized>(&'a self, key: &T) -> Option<BorrowedRef<'a, ()>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        for entry in entries {
            // SAFETY: The entry uses the same `root` so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.core.root(), entry.name) };
            if key == key_ref.get_ref() {
                // SAFETY: The ref uses the same `root` so the index is valid.
                return Some(unsafe { BorrowedRef::new(self.core.root(), entry.contents) });
            }
        }
        None
    }

    /// Get a mutable reference to the entry with the given key, if it exists.
    ///
    /// Returns `None` if no entry with the given key exists.
    #[must_use]
    pub fn get_mut<'a, T: PartialEq<MStr> + ?Sized>(
        &'a mut self,
        key: &T,
    ) -> Option<BorrowedMut<'a, ()>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        for entry in entries {
            // SAFETY: The entry uses the same `root` so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.core.root(), entry.name) };
            if key == key_ref.get_ref() {
                // SAFETY: The ref uses the same `root` so the index is valid.
                let index = entry.contents;
                return Some(unsafe { BorrowedMut::new(self.core.root_mut(), index) });
            }
        }
        None
    }

    /// Get a reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<BorrowedEntryRef<'_>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        entries.get(index).copied().map(|entry| BorrowedEntryRef { root: self.core.root(), entry })
    }

    /// Get a mutable reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<BorrowedEntryMut<'_>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        entries
            .get(index)
            .copied()
            .map(|entry| BorrowedEntryMut { root: self.core.root_mut(), entry })
    }

    /// Get an iterator over all entries in this compound.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedEntryRef<'_>> {
        // SAFETY: `range` is always guaranteed to be within bounds
        let entries = unsafe { self.core.entries().get_unchecked(self.range) };

        entries.iter().copied().map(|entry| BorrowedEntryRef { root: self.core.root(), entry })
    }

    /// Reborrow this [`IndexedCompoundMut`] with a shorter lifetime.
    #[inline]
    #[must_use]
    pub const fn reborrow(&mut self) -> IndexedCompoundMut<'_> {
        IndexedCompoundMut { core: self.core.reborrow(), range: self.range }
    }

    /// Get this [`IndexedCompoundMut`] as a [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> IndexedCompoundRef<'_> {
        IndexedCompoundRef { core: self.core.as_ref(), range: self.range }
    }

    /// Convert this [`IndexedCompoundMut`] into an [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub fn into_ref(self) -> IndexedCompoundRef<'data> {
        IndexedCompoundRef { core: self.core.into_ref(), range: self.range }
    }
}

impl BorrowedEntryMut<'_> {
    /// Get a reference to the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> BorrowedRef<'_, MStr> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedRef::new(self.root, self.entry.name) }
    }

    /// Get a mutable reference to the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name_mut(&mut self) -> BorrowedMut<'_, MStr> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedMut::new(self.root, self.entry.name) }
    }

    /// Get a mutable reference to the contents of this entry.
    #[inline]
    #[must_use]
    pub const fn contents(&self) -> BorrowedRef<'_, ()> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedRef::new(self.root, self.entry.contents) }
    }

    /// Get a mutable reference to the contents of this entry.
    #[inline]
    #[must_use]
    pub const fn contents_mut(&mut self) -> BorrowedMut<'_, ()> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedMut::new(self.root, self.entry.contents) }
    }
}

// -------------------------------------------------------------------------------------------------

/// An entry in an [`IndexedCompound`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedEntry {
    name: BorrowedIndex<MStr>,
    contents: BorrowedIndex<()>,
}

impl IndexedEntry {
    /// Create a new [`IndexedEntry`] with the given name and contents.
    #[must_use]
    pub const fn new(name: BorrowedIndex<MStr>, contents: BorrowedIndex<()>) -> Self {
        Self { name, contents }
    }
}
