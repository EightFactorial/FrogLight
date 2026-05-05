//! TODO

use core::{fmt, num::NonZeroUsize};

use froglight_mutf8::prelude::MStr;

use crate::types::borrowed::{
    IndexedCore,
    reference::{BorrowedIndex, BorrowedMut, BorrowedRef},
    value::{BorrowedValueIndex, BorrowedValueMut, BorrowedValueRef, BorrowedValueType},
};

/// An NBT compound tag with a reference to its data.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IndexedCompoundRef<'data> {
    root: &'data [u8],
    core: &'data IndexedCore,
    index: usize,
}

/// A reference to an entry in an [`IndexedCompound`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BorrowedEntryRef<'data> {
    root: &'data [u8],
    core: &'data IndexedCore,
    entry: IndexedEntry,
    index: Option<NonZeroUsize>,
}

impl<'data> IndexedCompoundRef<'data> {
    /// Create a new [`IndexedCompoundRef`] with the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index of `core`.
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data [u8],
        core: &'data IndexedCore,
        index: usize,
    ) -> Self {
        Self { root, core, index }
    }

    /// Get the number of entries in this [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };

        range.end.saturating_sub(range.start)
    }

    /// Returns `true` if the compound is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get a reference to the entry with the given key, if it exists.
    ///
    /// Returns `None` if no entry with the given key exists.
    #[must_use]
    pub fn get<T: PartialEq<MStr> + ?Sized>(&self, key: &T) -> Option<BorrowedValueRef<'data>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        for (i, entry) in entries.iter().enumerate() {
            // SAFETY: The entry uses the same `root` so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.root, entry.name()) };

            if key == key_ref.get_ref() {
                // SAFETY: `i` is a valid index of `entries`.
                let index = unsafe { get_index(self.index, i, entries) };

                // SAFETY: The ref uses the same `root` so the index is valid.
                return Some(unsafe {
                    BorrowedValueRef::new(self.root, self.core, entry.value(), index)
                });
            }
        }
        None
    }

    /// Get a reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<BorrowedEntryRef<'data>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        entries.get(index).copied().map(|entry| {
            // SAFETY: `i` is a valid index of `entries`.
            let index = unsafe { get_index(self.index, index, entries) };

            BorrowedEntryRef {
                entry,
                root: self.root,
                core: self.core,
                index: NonZeroUsize::new(index),
            }
        })
    }

    /// Get an iterator over all entries in this compound.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedEntryRef<'data>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        entries.iter().copied().enumerate().map(|(i, entry)| {
            // SAFETY: `i` is a valid index of `entries`.
            let index = unsafe { get_index(self.index, i, entries) };

            BorrowedEntryRef {
                entry,
                root: self.root,
                core: self.core,
                index: NonZeroUsize::new(index),
            }
        })
    }
}

impl<'data> BorrowedEntryRef<'data> {
    /// Get a reference to the name of this entry.
    #[inline]
    #[must_use]
    pub fn name(&self) -> BorrowedRef<'data, MStr> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedRef::new(self.root, self.entry.name()) }
    }

    /// Get a reference to the value of this entry.
    #[inline]
    #[must_use]
    pub fn value(&self) -> BorrowedValueRef<'data> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe {
            BorrowedValueRef::new_nonzero(self.root, self.core, self.entry.value(), self.index)
        }
    }
}

impl fmt::Debug for IndexedCompoundRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for entry in self.iter() {
            map.entry(&entry.name().get_ref(), &entry.value());
        }
        map.finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// An NBT compound tag with a mutable reference to its data.
#[derive(PartialEq, Eq)]
pub struct IndexedCompoundMut<'data> {
    root: &'data mut [u8],
    core: &'data IndexedCore,
    index: usize,
}

/// A mutable reference to an entry in an [`IndexedCompound`].
#[derive(PartialEq, Eq)]
pub struct BorrowedEntryMut<'data> {
    root: &'data mut [u8],
    core: &'data IndexedCore,
    entry: IndexedEntry,
    index: Option<NonZeroUsize>,
}

impl<'data> IndexedCompoundMut<'data> {
    /// Create a new [`IndexedCompound`] with the given root and entries.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is a valid index of `core`.
    #[must_use]
    pub(super) const unsafe fn new(
        root: &'data mut [u8],
        core: &'data IndexedCore,
        index: usize,
    ) -> Self {
        Self { root, core, index }
    }

    /// Get the number of entries in this [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };

        range.end.saturating_sub(range.start)
    }

    /// Returns `true` if the compound is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get a reference to the entry with the given key, if it exists.
    ///
    /// Returns `None` if no entry with the given key exists.
    #[must_use]
    pub fn get<'a, T: PartialEq<MStr> + ?Sized>(&'a self, key: &T) -> Option<BorrowedValueRef<'a>> {
        // SAFETY: `range` and `index` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        for (i, entry) in entries.iter().enumerate() {
            // SAFETY: The entry uses the same `root` so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.root, entry.name()) };

            if key == key_ref.get_ref() {
                // SAFETY: `i` is a valid index of `entries`.
                let index = unsafe { get_index(self.index, i, entries) };

                // SAFETY: The ref uses the same `root` so the index is valid.
                return Some(unsafe {
                    BorrowedValueRef::new(self.root, self.core, entry.value(), index)
                });
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
    ) -> Option<BorrowedValueMut<'a>> {
        // SAFETY: `range` and `index` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        for (i, entry) in entries.iter().enumerate() {
            // SAFETY: The entry uses the same `root` so the index is valid.
            let key_ref = unsafe { BorrowedRef::new(self.root, entry.name()) };

            if key == key_ref.get_ref() {
                // SAFETY: `i` is a valid index of `entries`.
                let index = unsafe { get_index(self.index, i, entries) };

                // SAFETY: The ref uses the same `root` so the index is valid.
                let value = entry.value();
                return Some(unsafe { BorrowedValueMut::new(self.root, self.core, value, index) });
            }
        }
        None
    }

    /// Get a reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<BorrowedEntryRef<'_>> {
        // SAFETY: `range` and `index` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        entries.get(index).copied().map(|entry| {
            // SAFETY: `i` is a valid index of `entries`.
            let index = unsafe { get_index(self.index, index, entries) };

            BorrowedEntryRef {
                entry,
                root: self.root,
                core: self.core,
                index: NonZeroUsize::new(index),
            }
        })
    }

    /// Get a mutable reference to the entry at the given index, if it exists.
    ///
    /// Returns `None` if no entry at the given index exists.
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<BorrowedEntryMut<'_>> {
        // SAFETY: `range` and `index` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        if let Some(entry) = entries.get(index).copied() {
            // SAFETY: `i` is a valid index of `entries`.
            let index = unsafe { get_index(self.index, index, entries) };

            // SAFETY: The ref uses the same `root` so the index is valid.
            Some(BorrowedEntryMut {
                entry,
                root: self.root,
                core: self.core,
                index: NonZeroUsize::new(index),
            })
        } else {
            None
        }
    }

    /// Get an iterator over all entries in this compound.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedEntryRef<'_>> {
        // SAFETY: `range` and `index` are always guaranteed to be within bounds
        let entries = unsafe {
            let range = self.core.indexes().get_unchecked(self.index);
            self.core.entries().get_unchecked(*range)
        };

        entries.iter().copied().enumerate().map(|(i, entry)| {
            // SAFETY: `i` is a valid index of `entries`.
            let index = unsafe { get_index(self.index, i, entries) };
            BorrowedEntryRef {
                entry,
                root: self.root,
                core: self.core,
                index: NonZeroUsize::new(index),
            }
        })
    }

    /// Reborrow this [`IndexedCompoundMut`] with a shorter lifetime.
    #[inline]
    #[must_use]
    pub const fn reborrow(&mut self) -> IndexedCompoundMut<'_> {
        IndexedCompoundMut { root: self.root, core: self.core, index: self.index }
    }

    /// Get this [`IndexedCompoundMut`] as a [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> IndexedCompoundRef<'_> {
        IndexedCompoundRef { root: self.root, core: self.core, index: self.index }
    }

    /// Convert this [`IndexedCompoundMut`] into an [`IndexedCompoundRef`].
    #[inline]
    #[must_use]
    pub fn into_ref(self) -> IndexedCompoundRef<'data> {
        IndexedCompoundRef { root: self.root, core: self.core, index: self.index }
    }
}

impl BorrowedEntryMut<'_> {
    /// Get a reference to the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> BorrowedRef<'_, MStr> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedRef::new(self.root, self.entry.name()) }
    }

    /// Get a mutable reference to the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name_mut(&mut self) -> BorrowedMut<'_, MStr> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe { BorrowedMut::new(self.root, self.entry.name()) }
    }

    /// Get a mutable reference to the value of this entry.
    #[inline]
    #[must_use]
    pub const fn value(&self) -> BorrowedValueRef<'_> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe {
            BorrowedValueRef::new_nonzero(self.root, self.core, self.entry.value(), self.index)
        }
    }

    /// Get a mutable reference to the value of this entry.
    #[inline]
    #[must_use]
    pub const fn value_mut(&mut self) -> BorrowedValueMut<'_> {
        // SAFETY: This entry corresponds to this `root`, so the index is valid.
        unsafe {
            BorrowedValueMut::new_nonzero(self.root, self.core, self.entry.value(), self.index)
        }
    }
}

impl fmt::Debug for IndexedCompoundMut<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for index in 0..self.len() {
            if let Some(entry) = self.get_index(index) {
                map.entry(&entry.name().get_ref(), &entry.value());
            }
        }
        map.finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// An entry in an [`IndexedCompound`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedEntry {
    name: BorrowedIndex<MStr>,
    value: BorrowedValueIndex,
}

impl IndexedEntry {
    /// Create a new [`IndexedEntry`] with the given name and contents.
    #[must_use]
    pub const fn new(name: BorrowedIndex<MStr>, value: BorrowedValueIndex) -> Self {
        Self { name, value }
    }

    /// Get the name of this entry.
    #[must_use]
    pub const fn name(&self) -> BorrowedIndex<MStr> { self.name }

    /// Get the value of this entry.
    #[must_use]
    pub const fn value(&self) -> BorrowedValueIndex { self.value }
}

// -------------------------------------------------------------------------------------------------

/// Get the index of the entry at `entry_index`.
///
/// # Safety
///
/// `entry_index` must be a valid index of `entries`.
#[inline]
pub(super) unsafe fn get_index(
    compound_index: usize,
    entry_index: usize,
    entries: &[IndexedEntry],
) -> usize {
    // SAFETY: `entry_index` is a valid index of `entries`
    let entry = unsafe { entries.get_unchecked(entry_index) };

    // Only `Compound` and `List` entries have indexes.
    if !matches!(entry.value().ty(), BorrowedValueType::Compound | BorrowedValueType::List) {
        return 0;
    }

    entries.iter().take(entry_index).fold(compound_index.max(1), |index, entry| {
        index
            + usize::from(matches!(
                entry.value().ty(),
                BorrowedValueType::Compound | BorrowedValueType::List
            ))
    })
}
