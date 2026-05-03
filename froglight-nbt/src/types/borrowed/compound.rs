//! TODO

use froglight_mutf8::prelude::MStr;

use crate::types::borrowed::reference::{BorrowedIndex, BorrowedMut, BorrowedRef};

/// An NBT compound tag with a reference to its data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedCompoundRef<'data> {
    root: &'data [u8],
    entries: &'data [IndexedEntry],
}

/// A reference to an entry in an [`IndexedCompound`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedEntryRef<'data> {
    root: &'data [u8],
    entry: &'data IndexedEntry,
}

impl<'data> IndexedCompoundRef<'data> {
    /// Create a new [`IndexedCompoundRef`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `entries` are valid indexes into `root`.
    #[must_use]
    pub const unsafe fn new(root: &'data [u8], entries: &'data [IndexedEntry]) -> Self {
        Self { root, entries }
    }

    /// Get the number of entries in this [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.entries.len() }

    /// Returns `true` if the compound is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.entries.is_empty() }

    /// Get a reference to the entry with the given key, if it exists.
    ///
    /// Returns `None` if no entry with the given key exists.
    #[must_use]
    pub fn get<'a, T: PartialEq<MStr> + ?Sized>(&'a self, key: &T) -> Option<BorrowedRef<'a, ()>> {
        for entry in self.entries {
            // SAFETY: This entry corresponds to this `root`, so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.root, entry.name) };
            if key == key_ref.get_ref() {
                // SAFETY: This entry corresponds to this `root`, so the index is valid.
                return Some(unsafe { BorrowedRef::new(self.root, entry.contents) });
            }
        }
        None
    }

    /// Get a reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<BorrowedEntryRef<'_>> {
        self.entries.get(index).map(|entry| BorrowedEntryRef { root: self.root, entry })
    }

    /// Get an iterator over all entries in this compound.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedEntryRef<'_>> {
        self.entries.iter().map(|entry| BorrowedEntryRef { root: self.root, entry })
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
    root: &'data mut [u8],
    entries: &'data [IndexedEntry],
}

/// A mutable reference to an entry in an [`IndexedCompound`].
#[derive(Debug, PartialEq, Eq)]
pub struct BorrowedEntryMut<'data> {
    root: &'data mut [u8],
    entry: &'data IndexedEntry,
}

impl<'data> IndexedCompoundMut<'data> {
    /// Create a new [`IndexedCompound`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `entries` are valid indexes into `root`.
    #[must_use]
    pub const unsafe fn new(root: &'data mut [u8], entries: &'data [IndexedEntry]) -> Self {
        Self { root, entries }
    }

    /// Get the number of entries in this [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.entries.len() }

    /// Returns `true` if the compound is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.entries.is_empty() }

    /// Get a reference to the entry with the given key, if it exists.
    ///
    /// Returns `None` if no entry with the given key exists.
    #[must_use]
    pub fn get<'a, T: PartialEq<MStr> + ?Sized>(&'a self, key: &T) -> Option<BorrowedRef<'a, ()>> {
        for entry in self.entries {
            // SAFETY: This entry corresponds to this `root`, so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.root, entry.name) };
            if key == key_ref.get_ref() {
                // SAFETY: This entry corresponds to this `root`, so the index is valid.
                return Some(unsafe { BorrowedRef::new(self.root, entry.contents) });
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
        for entry in self.entries {
            // SAFETY: This entry corresponds to this `root`, so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.root, entry.name) };
            if key == key_ref.get_ref() {
                // SAFETY: This entry corresponds to this `root`, so the index is valid.
                return Some(unsafe { BorrowedMut::new(self.root, entry.contents) });
            }
        }
        None
    }

    /// Get a reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<BorrowedEntryRef<'_>> {
        self.entries.get(index).map(|entry| BorrowedEntryRef { root: self.root, entry })
    }

    /// Get a mutable reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<BorrowedEntryMut<'_>> {
        self.entries.get(index).map(|entry| BorrowedEntryMut { root: self.root, entry })
    }

    /// Get an iterator over all entries in this compound.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedEntryRef<'_>> {
        self.entries.iter().map(|entry| BorrowedEntryRef { root: self.root, entry })
    }

    /// Get this [`IndexedCompoundMut`] as a [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> IndexedCompoundRef<'_> {
        IndexedCompoundRef { root: self.root, entries: self.entries }
    }

    /// Convert this [`IndexedCompoundMut`] into a [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn into_ref(self) -> IndexedCompoundRef<'data> {
        IndexedCompoundRef { root: self.root, entries: self.entries }
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
