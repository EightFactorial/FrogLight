//! TODO

use core::{fmt, marker::PhantomData, range::Range};

use crate::types::borrowed::{
    IndexedCore,
    reference::{BorrowedIndex, BorrowedMut, BorrowedPOD, BorrowedRef},
    value::{BorrowedValueRef, IndexedList},
};

/// An NBT list tag with a reference to its data.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IndexedListRef<'data, T> {
    root: &'data [u8],
    core: &'data IndexedCore,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'data, T> IndexedListRef<'data, T> {
    /// Create a new [`IndexedListRef`] with the given core and index.
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
        Self { root, core, index, _phantom: PhantomData }
    }

    /// Get the number of entries in this [`IndexedListRef`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };

        range.end.saturating_sub(range.start)
    }

    /// Returns `true` if the list is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

impl<T: BorrowedPOD> IndexedListRef<'_, T> {
    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_value(&self, index: usize) -> Option<T> {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let normalized = Range { start: 0, end: range.end.saturating_sub(range.start) };

        if normalized.contains(&index) {
            let data_index = range.start + (core::mem::size_of::<T>() * index);
            Some(unsafe { BorrowedRef::new(self.root, BorrowedIndex::new(data_index)).get_value() })
        } else {
            None
        }
    }
}

impl IndexedListRef<'_, IndexedList> {
    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<BorrowedValueRef<'_>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        if let Some(entry) = entries.get(index) {
            let ref_index = unsafe { super::compound::get_index(self.index, index, entries) };
            Some(unsafe { BorrowedValueRef::new(self.root, self.core, entry.value(), ref_index) })
        } else {
            None
        }
    }

    /// Get an iterator over all entries in this list.
    pub fn iter(&self) -> impl Iterator<Item = BorrowedValueRef<'_>> + '_ {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        entries.iter().enumerate().map(move |(index, entry)| {
            let ref_index = unsafe { super::compound::get_index(self.index, index, entries) };
            unsafe { BorrowedValueRef::new(self.root, self.core, entry.value(), ref_index) }
        })
    }
}

impl<T: fmt::Debug + BorrowedPOD> fmt::Debug for IndexedListRef<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for index in 0..self.len() {
            if let Some(value) = self.get_value(index) {
                list.entry(&value);
            }
        }
        list.finish()
    }
}

impl fmt::Debug for IndexedListRef<'_, IndexedList> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// An NBT list tag with a mutable reference to its data.
#[derive(PartialEq, Eq)]
pub struct IndexedListMut<'data, T> {
    root: &'data mut [u8],
    core: &'data IndexedCore,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'data, T> IndexedListMut<'data, T> {
    /// Create a new [`IndexedListMut`] with the given core and index.
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
        Self { root, core, index, _phantom: PhantomData }
    }

    /// Get the number of entries in this [`IndexedListMut`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `index` is always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };

        range.end.saturating_sub(range.start)
    }

    /// Returns `true` if the list is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

impl<T: BorrowedPOD> IndexedListMut<'_, T> {
    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_value(&self, index: usize) -> Option<T> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        if let Some(entry) = entries.get(index) {
            // SAFETY: `index` is valid for `core` and is of type `T`
            let index = unsafe { BorrowedIndex::new(entry.value().index()) };
            Some(unsafe { BorrowedRef::new(self.root, index).get_value() })
        } else {
            None
        }
    }

    /// Set the value at the provided index, if it exists.
    ///
    /// Returns `true` if the value was successfully set,
    /// or `false` if the index is out of bounds.
    pub fn set_value(&mut self, index: usize, value: T) -> bool {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        if let Some(entry) = entries.get(index) {
            // SAFETY: `index` is valid for `core` and is of type `T`
            let index = unsafe { BorrowedIndex::new(entry.value().index()) };
            unsafe { BorrowedMut::new(self.root, index).set_value(value) }
            true
        } else {
            false
        }
    }
}

impl IndexedListMut<'_, IndexedList> {
    /// Get the value at the provided index, if it exists.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<BorrowedValueRef<'_>> {
        // SAFETY: `index` and `range` are always guaranteed to be within bounds
        let range = unsafe { self.core.indexes().get_unchecked(self.index) };
        let entries = unsafe { self.core.entries().get_unchecked(*range) };

        if let Some(entry) = entries.get(index) {
            let ref_index = unsafe { super::compound::get_index(self.index, index, entries) };
            Some(unsafe { BorrowedValueRef::new(self.root, self.core, entry.value(), ref_index) })
        } else {
            None
        }
    }
}

impl<T: fmt::Debug + BorrowedPOD> fmt::Debug for IndexedListMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for index in 0..self.len() {
            if let Some(value) = self.get_value(index) {
                list.entry(&value);
            }
        }
        list.finish()
    }
}

impl fmt::Debug for IndexedListMut<'_, IndexedList> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for index in 0..self.len() {
            if let Some(value) = self.get(index) {
                list.entry(&value);
            }
        }
        list.finish()
    }
}
